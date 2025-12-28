//! Vault Contract for Casper Predict.
//!
//! The Vault is a secure escrow contract that holds all CSPR funds for
//! prediction markets. It provides the following guarantees:
//!
//! - **Isolation**: Each market's funds are tracked separately
//! - **Authorization**: Only authorized market contracts can deposit/withdraw
//! - **Security**: Admin controls for emergency situations
//! - **Fee Collection**: Platform fees are accumulated and claimable

use odra::prelude::*;
use odra::casper_types::{U256, U512};

use crate::errors::Error;
use crate::events::{
    FeesClaimed, FeesCollected, FundsDeposited, FundsWithdrawn,
    MarketAuthorized, MarketRevoked, VaultPauseStatusChanged, AdminTransferred,
};

/// Convert U512 to U256 (assumes value fits)
fn u512_to_u256(value: U512) -> U256 {
    // Extract the lower 256 bits
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = value.byte(i);
    }
    U256::from_little_endian(&bytes)
}

/// Convert U256 to U512
fn u256_to_u512(value: U256) -> U512 {
    let mut bytes = [0u8; 32];
    value.to_little_endian(&mut bytes);
    U512::from_little_endian(&bytes)
}

/// The Vault contract - secure escrow for all prediction market funds.
#[odra::module(events = [
    FundsDeposited, FundsWithdrawn, FeesCollected, FeesClaimed,
    MarketAuthorized, MarketRevoked, VaultPauseStatusChanged, AdminTransferred
])]
pub struct Vault {
    // =========================================================================
    // Admin Controls
    // =========================================================================
    /// Contract administrator
    admin: Var<Address>,
    /// Emergency pause flag
    paused: Var<bool>,
    /// Whether the contract has been initialized
    initialized: Var<bool>,

    // =========================================================================
    // Market Balances
    // =========================================================================
    /// Balance per market (market_id -> CSPR balance in motes)
    market_balances: Mapping<u64, U256>,
    /// Total CSPR locked across all markets
    total_locked: Var<U256>,

    // =========================================================================
    // Authorization
    // =========================================================================
    /// Authorized market contracts (address -> is_authorized)
    authorized_markets: Mapping<Address, bool>,
    /// Factory contract address (can authorize markets)
    factory_contract: Var<Address>,

    // =========================================================================
    // Fee Collection
    // =========================================================================
    /// Total platform fees collected and available for claiming
    platform_fees_collected: Var<U256>,
    /// Address that receives platform fees
    fee_recipient: Var<Address>,
}

#[odra::module]
impl Vault {
    /// Initialize the vault with admin and fee recipient addresses.
    pub fn init(&mut self, admin: Address, fee_recipient: Address) {
        if self.initialized.get_or_default() {
            self.env().revert(Error::AlreadyInitialized);
        }

        self.admin.set(admin);
        self.fee_recipient.set(fee_recipient);
        self.paused.set(false);
        self.total_locked.set(U256::zero());
        self.platform_fees_collected.set(U256::zero());
        self.initialized.set(true);
    }

    // =========================================================================
    // Core Deposit/Withdraw Functions
    // =========================================================================

    /// Deposit CSPR into the vault for a specific market.
    /// 
    /// This function must be called by an authorized market contract.
    /// The CSPR is attached to the transaction.
    #[odra(payable)]
    pub fn deposit(&mut self, market_id: u64) {
        self.require_not_paused();
        self.require_authorized_caller();

        let amount_u512 = self.env().attached_value();
        let amount = u512_to_u256(amount_u512);
        
        if amount.is_zero() {
            self.env().revert(Error::ZeroAmount);
        }

        // Update market balance
        let current_balance = self.market_balances.get(&market_id).unwrap_or_default();
        let new_balance = current_balance.saturating_add(amount);
        self.market_balances.set(&market_id, new_balance);

        // Update total locked
        let total = self.total_locked.get_or_default();
        self.total_locked.set(total.saturating_add(amount));

        self.env().emit_event(FundsDeposited {
            market_id,
            amount,
            from_contract: self.env().caller(),
        });
    }

    /// Withdraw CSPR from the vault for a specific market.
    ///
    /// This function must be called by an authorized market contract.
    /// The CSPR is transferred to the specified recipient.
    pub fn withdraw(&mut self, market_id: u64, recipient: Address, amount: U256) {
        self.require_not_paused();
        self.require_authorized_caller();

        if amount.is_zero() {
            self.env().revert(Error::ZeroAmount);
        }

        // Check market balance
        let current_balance = self.market_balances.get(&market_id).unwrap_or_default();
        if current_balance < amount {
            self.env().revert(Error::ExceedsMarketBalance);
        }

        // Update market balance
        let new_balance = current_balance.saturating_sub(amount);
        self.market_balances.set(&market_id, new_balance);

        // Update total locked
        let total = self.total_locked.get_or_default();
        self.total_locked.set(total.saturating_sub(amount));

        // Transfer CSPR to recipient
        let amount_u512 = u256_to_u512(amount);
        self.env().transfer_tokens(&recipient, &amount_u512);

        self.env().emit_event(FundsWithdrawn {
            market_id,
            recipient,
            amount,
        });
    }

    // =========================================================================
    // Fee Management
    // =========================================================================

    /// Collect platform fees from a market's balance.
    ///
    /// Called by market contracts when fees are deducted from trades.
    pub fn collect_platform_fees(&mut self, market_id: u64, fee_amount: U256) {
        self.require_not_paused();
        self.require_authorized_caller();

        if fee_amount.is_zero() {
            return;
        }

        // Deduct from market balance
        let current_balance = self.market_balances.get(&market_id).unwrap_or_default();
        if current_balance < fee_amount {
            self.env().revert(Error::ExceedsMarketBalance);
        }

        let new_balance = current_balance.saturating_sub(fee_amount);
        self.market_balances.set(&market_id, new_balance);

        // Add to collected fees
        let fees = self.platform_fees_collected.get_or_default();
        self.platform_fees_collected.set(fees.saturating_add(fee_amount));

        self.env().emit_event(FeesCollected {
            market_id,
            amount: fee_amount,
        });
    }

    /// Claim all collected platform fees.
    ///
    /// Can only be called by the fee recipient.
    pub fn claim_platform_fees(&mut self) {
        self.require_not_paused();

        let caller = self.env().caller();
        let recipient = self.fee_recipient.get().unwrap_or_revert(&self.env());

        if caller != recipient {
            self.env().revert(Error::AccessDenied);
        }

        let amount = self.platform_fees_collected.get_or_default();
        if amount.is_zero() {
            self.env().revert(Error::NothingToClaim);
        }

        // Reset collected fees
        self.platform_fees_collected.set(U256::zero());

        // Update total locked
        let total = self.total_locked.get_or_default();
        self.total_locked.set(total.saturating_sub(amount));

        // Transfer fees
        let amount_u512 = u256_to_u512(amount);
        self.env().transfer_tokens(&recipient, &amount_u512);

        self.env().emit_event(FeesClaimed {
            recipient,
            amount,
        });
    }

    // =========================================================================
    // Authorization Management
    // =========================================================================

    /// Authorize a market contract to interact with the vault.
    ///
    /// Can be called by admin or the factory contract.
    pub fn authorize_market(&mut self, market: Address) {
        let caller = self.env().caller();
        let admin = self.admin.get().unwrap_or_revert(&self.env());
        let factory = self.factory_contract.get();

        let is_admin = caller == admin;
        let is_factory = factory.map(|f| f == caller).unwrap_or(false);

        if !is_admin && !is_factory {
            self.env().revert(Error::AccessDenied);
        }

        if self.authorized_markets.get(&market).unwrap_or(false) {
            self.env().revert(Error::MarketAlreadyAuthorized);
        }

        self.authorized_markets.set(&market, true);

        self.env().emit_event(MarketAuthorized { market });
    }

    /// Revoke authorization for a market contract.
    ///
    /// Can only be called by admin.
    pub fn revoke_market(&mut self, market: Address) {
        self.require_admin();

        if !self.authorized_markets.get(&market).unwrap_or(false) {
            self.env().revert(Error::MarketNotAuthorized);
        }

        self.authorized_markets.set(&market, false);

        self.env().emit_event(MarketRevoked { market });
    }

    /// Set the factory contract address.
    ///
    /// Can only be called by admin.
    pub fn set_factory(&mut self, factory: Address) {
        self.require_admin();
        self.factory_contract.set(factory);
    }

    // =========================================================================
    // Admin Functions
    // =========================================================================

    /// Pause the vault (emergency use only).
    pub fn pause(&mut self) {
        self.require_admin();
        self.paused.set(true);
        self.env().emit_event(VaultPauseStatusChanged { paused: true });
    }

    /// Unpause the vault.
    pub fn unpause(&mut self) {
        self.require_admin();
        self.paused.set(false);
        self.env().emit_event(VaultPauseStatusChanged { paused: false });
    }

    /// Transfer admin role to a new address.
    pub fn transfer_admin(&mut self, new_admin: Address) {
        self.require_admin();
        let previous_admin = self.admin.get().unwrap_or_revert(&self.env());
        self.admin.set(new_admin);
        self.env().emit_event(AdminTransferred {
            previous_admin,
            new_admin,
        });
    }

    /// Update the fee recipient address.
    pub fn update_fee_recipient(&mut self, new_recipient: Address) {
        self.require_admin();
        self.fee_recipient.set(new_recipient);
    }

    // =========================================================================
    // View Functions
    // =========================================================================

    /// Get the CSPR balance for a specific market.
    pub fn get_market_balance(&self, market_id: u64) -> U256 {
        self.market_balances.get(&market_id).unwrap_or_default()
    }

    /// Get the total CSPR locked in the vault.
    pub fn get_total_locked(&self) -> U256 {
        self.total_locked.get_or_default()
    }

    /// Get the total unclaimed platform fees.
    pub fn get_platform_fees(&self) -> U256 {
        self.platform_fees_collected.get_or_default()
    }

    /// Check if a market is authorized.
    pub fn is_market_authorized(&self, market: Address) -> bool {
        self.authorized_markets.get(&market).unwrap_or(false)
    }

    /// Check if the vault is paused.
    pub fn is_paused(&self) -> bool {
        self.paused.get_or_default()
    }

    /// Get the admin address.
    pub fn get_admin(&self) -> Address {
        self.admin.get().unwrap_or_revert(&self.env())
    }

    /// Get the fee recipient address.
    pub fn get_fee_recipient(&self) -> Address {
        self.fee_recipient.get().unwrap_or_revert(&self.env())
    }

    /// Get the factory contract address.
    pub fn get_factory(&self) -> Option<Address> {
        self.factory_contract.get()
    }

    // =========================================================================
    // Internal Helper Functions
    // =========================================================================

    /// Ensure the caller is the admin.
    fn require_admin(&self) {
        let caller = self.env().caller();
        let admin = self.admin.get().unwrap_or_revert(&self.env());
        if caller != admin {
            self.env().revert(Error::NotAdmin);
        }
    }

    /// Ensure the vault is not paused.
    fn require_not_paused(&self) {
        if self.paused.get_or_default() {
            self.env().revert(Error::VaultPaused);
        }
    }

    /// Ensure the caller is an authorized market or the factory.
    fn require_authorized_caller(&self) {
        let caller = self.env().caller();
        
        // Check if caller is an authorized market
        if self.authorized_markets.get(&caller).unwrap_or(false) {
            return;
        }

        // Check if caller is the factory
        if let Some(factory) = self.factory_contract.get() {
            if factory == caller {
                return;
            }
        }

        // Check if caller is admin (for testing/setup)
        if let Some(admin) = self.admin.get() {
            if admin == caller {
                return;
            }
        }

        self.env().revert(Error::UnauthorizedMarket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, HostRef, NoArgs};

    fn setup() -> (VaultHostRef, odra::host::HostEnv) {
        let env = odra_test::env();
        let admin = env.get_account(0);
        let fee_recipient = env.get_account(1);
        
        let init_args = VaultInitArgs {
            admin,
            fee_recipient,
        };
        
        let vault = Vault::deploy(&env, init_args);
        (vault, env)
    }

    #[test]
    fn test_init() {
        let (vault, env) = setup();
        let admin = env.get_account(0);
        let fee_recipient = env.get_account(1);
        
        assert_eq!(vault.get_admin(), admin);
        assert_eq!(vault.get_fee_recipient(), fee_recipient);
        assert!(!vault.is_paused());
        assert_eq!(vault.get_total_locked(), U256::zero());
    }

    #[test]
    fn test_pause_unpause() {
        let (mut vault, _env) = setup();
        
        assert!(!vault.is_paused());
        
        vault.pause();
        assert!(vault.is_paused());
        
        vault.unpause();
        assert!(!vault.is_paused());
    }

    #[test]
    fn test_authorize_revoke_market() {
        let (mut vault, env) = setup();
        let market = env.get_account(2);
        
        assert!(!vault.is_market_authorized(market));
        
        vault.authorize_market(market);
        assert!(vault.is_market_authorized(market));
        
        vault.revoke_market(market);
        assert!(!vault.is_market_authorized(market));
    }
}
