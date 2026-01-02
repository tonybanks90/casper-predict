//! MarketFactory Contract for Casper Predict.
//!
//! The factory is responsible for:
//! - Creating new prediction markets (Binary, Multiple Choice, Compound)
//! - Maintaining a registry of all markets
//! - Setting global parameters (fees, duration limits, etc.)
//! - Emergency pause functionality

use odra::prelude::*;
use odra::casper_types::{U256, U512};

use crate::errors::Error;
use crate::types::{BondingCurveParams, MarketType};
use crate::events::{
    MarketCreated, PlatformFeeUpdated, FactoryPauseStatusChanged, AdminTransferred,
};

/// Maximum fee percentage (10% = 1000 basis points)
const MAX_FEE_BPS: u64 = 1000;

/// Convert U512 to U256 (assumes value fits)
fn u512_to_u256(value: U512) -> U256 {
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = value.byte(i);
    }
    U256::from_little_endian(&bytes)
}

/// MarketFactory - Central contract for creating prediction markets.
#[odra::module(
    errors = Error,
    events = [MarketCreated, PlatformFeeUpdated, FactoryPauseStatusChanged, AdminTransferred]
)]
pub struct MarketFactory {
    // =========================================================================
    // Admin Controls
    // =========================================================================
    /// Contract administrator
    admin: Var<Address>,
    /// Factory pause flag
    paused: Var<bool>,
    /// Whether the contract has been initialized
    initialized: Var<bool>,

    // =========================================================================
    // Market Registry
    // =========================================================================
    /// Counter for market IDs
    market_count: Var<u64>,
    /// Mapping of market_id to market data
    markets: Mapping<u64, MarketData>,

    // =========================================================================
    // Global Parameters
    // =========================================================================
    /// Platform fee in basis points (e.g., 200 = 2%)
    platform_fee_bps: Var<u64>,
    /// Minimum market duration in seconds
    min_market_duration: Var<u64>,
    /// Maximum market duration in seconds
    max_market_duration: Var<u64>,
    /// Minimum initial liquidity in motes
    min_initial_liquidity: Var<U256>,
    /// Default bonding curve parameters
    default_bonding_params: Var<BondingCurveParams>,

    // =========================================================================
    // Vault Reference
    // =========================================================================
    /// Vault contract address
    vault_contract: Var<Address>,
}

/// Stored data for each market.
#[odra::odra_type]
pub struct MarketData {
    /// Market ID
    pub market_id: u64,
    /// Market type
    pub market_type: MarketType,
    /// The prediction question
    pub question: String,
    /// Creator address
    pub creator: Address,
    /// End time
    pub end_time: u64,
    /// Category
    pub category: String,
    /// Creation timestamp
    pub created_at: u64,
}

/// Initialization arguments for the factory.
#[odra::odra_type]
pub struct FactoryInitArgs {
    pub admin: Address,
    pub vault_contract: Address,
    pub platform_fee_bps: u64,
    pub min_market_duration: u64,
    pub max_market_duration: u64,
    pub min_initial_liquidity: U256,
}

#[odra::module]
impl MarketFactory {
    /// Initialize the factory contract.
    pub fn init(&mut self, args: FactoryInitArgs) {
        if self.initialized.get_or_default() {
            self.env().revert(Error::AlreadyInitialized);
        }

        if args.platform_fee_bps > MAX_FEE_BPS {
            self.env().revert(Error::FeeTooHigh);
        }

        self.admin.set(args.admin);
        self.vault_contract.set(args.vault_contract);
        self.platform_fee_bps.set(args.platform_fee_bps);
        self.min_market_duration.set(args.min_market_duration);
        self.max_market_duration.set(args.max_market_duration);
        self.min_initial_liquidity.set(args.min_initial_liquidity);
        self.default_bonding_params.set(BondingCurveParams::default_params());
        
        self.paused.set(false);
        self.market_count.set(0);
        self.initialized.set(true);
    }

    // =========================================================================
    // Market Creation Functions
    // =========================================================================

    /// Create a new binary (YES/NO) prediction market.
    ///
    /// Returns the market ID.
    #[odra(payable)]
    pub fn create_binary_market(
        &mut self,
        question: String,
        end_time: u64,
        _resolution_source: String,
        category: String,
    ) -> u64 {
        self.require_not_paused();
        self.validate_market_params(&question, end_time);

        let initial_liquidity_u512 = self.env().attached_value();
        let initial_liquidity = u512_to_u256(initial_liquidity_u512);
        self.validate_initial_liquidity(initial_liquidity);

        let creator = self.env().caller();
        let market_id = self.next_market_id();

        // Store market data
        let market_data = MarketData {
            market_id,
            market_type: MarketType::Binary,
            question: question.clone(),
            creator,
            end_time,
            category: category.clone(),
            created_at: self.env().get_block_time(),
        };
        self.markets.set(&market_id, market_data);

        // Emit event
        self.env().emit_event(MarketCreated {
            market_id,
            market_type: 0, // Binary
            creator,
            question,
            end_time,
            category,
        });

        // Note: In a full implementation, we would deploy a separate Market contract
        // or use SubModules with dynamic creation. For now, we store the market data
        // and the frontend/users would interact with individual Market contracts.

        market_id
    }

    /// Create a new multiple choice prediction market.
    ///
    /// Returns the market ID.
    #[odra(payable)]
    pub fn create_multiple_choice_market(
        &mut self,
        question: String,
        outcomes: Vec<String>,
        end_time: u64,
        _resolution_source: String,
        category: String,
    ) -> u64 {
        self.require_not_paused();
        self.validate_market_params(&question, end_time);

        // Validate outcomes
        if outcomes.len() < 2 {
            self.env().revert(Error::InvalidOutcomeCount);
        }

        let initial_liquidity_u512 = self.env().attached_value();
        let initial_liquidity = u512_to_u256(initial_liquidity_u512);
        self.validate_initial_liquidity(initial_liquidity);

        let creator = self.env().caller();
        let market_id = self.next_market_id();

        // Store market data
        let market_data = MarketData {
            market_id,
            market_type: MarketType::MultipleChoice,
            question: question.clone(),
            creator,
            end_time,
            category: category.clone(),
            created_at: self.env().get_block_time(),
        };
        self.markets.set(&market_id, market_data);

        // Emit event
        self.env().emit_event(MarketCreated {
            market_id,
            market_type: 1, // MultipleChoice
            creator,
            question,
            end_time,
            category,
        });

        market_id
    }

    // =========================================================================
    // Admin Functions
    // =========================================================================

    /// Update the platform fee percentage.
    pub fn set_platform_fee(&mut self, new_fee_bps: u64) {
        self.require_admin();

        if new_fee_bps > MAX_FEE_BPS {
            self.env().revert(Error::FeeTooHigh);
        }

        let old_fee = self.platform_fee_bps.get_or_default();
        self.platform_fee_bps.set(new_fee_bps);

        self.env().emit_event(PlatformFeeUpdated {
            old_fee,
            new_fee: new_fee_bps,
        });
    }

    /// Update market duration limits.
    pub fn set_duration_limits(&mut self, min_duration: u64, max_duration: u64) {
        self.require_admin();

        if min_duration > max_duration {
            self.env().revert(Error::InvalidMarketDuration);
        }

        self.min_market_duration.set(min_duration);
        self.max_market_duration.set(max_duration);
    }

    /// Update minimum initial liquidity.
    pub fn set_min_initial_liquidity(&mut self, min_liquidity: U256) {
        self.require_admin();
        self.min_initial_liquidity.set(min_liquidity);
    }

    /// Update the vault contract address.
    pub fn update_vault_contract(&mut self, new_vault: Address) {
        self.require_admin();
        self.vault_contract.set(new_vault);
    }

    /// Update default bonding curve parameters.
    pub fn set_default_bonding_params(&mut self, params: BondingCurveParams) {
        self.require_admin();
        self.default_bonding_params.set(params);
    }

    /// Pause the factory (stops new market creation).
    pub fn pause(&mut self) {
        self.require_admin();
        self.paused.set(true);
        self.env().emit_event(FactoryPauseStatusChanged { paused: true });
    }

    /// Unpause the factory.
    pub fn unpause(&mut self) {
        self.require_admin();
        self.paused.set(false);
        self.env().emit_event(FactoryPauseStatusChanged { paused: false });
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

    // =========================================================================
    // View Functions
    // =========================================================================

    /// Get the total number of markets created.
    pub fn get_market_count(&self) -> u64 {
        self.market_count.get_or_default()
    }

    /// Get market data by ID.
    pub fn get_market_data(&self, market_id: u64) -> Option<MarketData> {
        self.markets.get(&market_id)
    }

    /// Get the platform fee in basis points.
    pub fn get_platform_fee(&self) -> u64 {
        self.platform_fee_bps.get_or_default()
    }

    /// Get minimum market duration.
    pub fn get_min_duration(&self) -> u64 {
        self.min_market_duration.get_or_default()
    }

    /// Get maximum market duration.
    pub fn get_max_duration(&self) -> u64 {
        self.max_market_duration.get_or_default()
    }

    /// Get minimum initial liquidity.
    pub fn get_min_initial_liquidity(&self) -> U256 {
        self.min_initial_liquidity.get_or_default()
    }

    /// Get the vault contract address.
    pub fn get_vault_contract(&self) -> Address {
        self.vault_contract.get().unwrap_or_revert(&self.env())
    }

    /// Get default bonding curve parameters.
    pub fn get_default_bonding_params(&self) -> BondingCurveParams {
        self.default_bonding_params.get_or_default()
    }

    /// Check if the factory is paused.
    pub fn is_paused(&self) -> bool {
        self.paused.get_or_default()
    }

    /// Get the admin address.
    pub fn get_admin(&self) -> Address {
        self.admin.get().unwrap_or_revert(&self.env())
    }

    // =========================================================================
    // Internal Helper Functions
    // =========================================================================

    /// Get the next market ID and increment the counter.
    fn next_market_id(&mut self) -> u64 {
        let current = self.market_count.get_or_default();
        self.market_count.set(current + 1);
        current
    }

    /// Ensure the caller is the admin.
    fn require_admin(&self) {
        let caller = self.env().caller();
        let admin = self.admin.get().unwrap_or_revert(&self.env());
        if caller != admin {
            self.env().revert(Error::NotAdmin);
        }
    }

    /// Ensure the factory is not paused.
    fn require_not_paused(&self) {
        if self.paused.get_or_default() {
            self.env().revert(Error::FactoryPaused);
        }
    }

    /// Validate market creation parameters.
    fn validate_market_params(&self, question: &str, end_time: u64) {
        // Check question is not empty and not too long
        if question.is_empty() || question.len() > 1000 {
            self.env().revert(Error::InvalidQuestion);
        }

        // Check duration is within limits
        let current_time = self.env().get_block_time();
        if end_time <= current_time {
            self.env().revert(Error::InvalidMarketDuration);
        }

        let duration = end_time - current_time;
        let min_duration = self.min_market_duration.get_or_default();
        let max_duration = self.max_market_duration.get_or_default();

        if duration < min_duration || duration > max_duration {
            self.env().revert(Error::InvalidMarketDuration);
        }
    }

    /// Validate initial liquidity.
    fn validate_initial_liquidity(&self, liquidity: U256) {
        let min_liquidity = self.min_initial_liquidity.get_or_default();
        if liquidity < min_liquidity {
            self.env().revert(Error::InsufficientInitialLiquidity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, HostRef};

    fn setup() -> (MarketFactoryHostRef, odra::host::HostEnv) {
        let env = odra_test::env();
        let admin = env.get_account(0);
        
        let init_args = FactoryInitArgs {
            admin,
            vault_contract: admin, // Placeholder
            platform_fee_bps: 200, // 2%
            min_market_duration: 3600, // 1 hour
            max_market_duration: 86400 * 30, // 30 days
            min_initial_liquidity: U256::from(1_000_000_000u64), // 1 CSPR
        };

        let factory = MarketFactory::deploy(&env, init_args);
        (factory, env)
    }

    #[test]
    fn test_factory_init() {
        let (factory, env) = setup();
        let admin = env.get_account(0);
        
        assert_eq!(factory.get_admin(), admin);
        assert_eq!(factory.get_platform_fee(), 200);
        assert_eq!(factory.get_market_count(), 0);
        assert!(!factory.is_paused());
    }

    #[test]
    fn test_pause_unpause() {
        let (mut factory, _env) = setup();
        
        assert!(!factory.is_paused());
        
        factory.pause();
        assert!(factory.is_paused());
        
        factory.unpause();
        assert!(!factory.is_paused());
    }

    #[test]
    fn test_set_platform_fee() {
        let (mut factory, _env) = setup();
        
        assert_eq!(factory.get_platform_fee(), 200);
        
        factory.set_platform_fee(300);
        assert_eq!(factory.get_platform_fee(), 300);
    }

    #[test]
    fn test_transfer_admin() {
        let (mut factory, env) = setup();
        let new_admin = env.get_account(1);
        
        factory.transfer_admin(new_admin);
        assert_eq!(factory.get_admin(), new_admin);
    }
}
