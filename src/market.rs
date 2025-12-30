//! Market Contract for Casper Predict.
//!
//! Each Market contract represents a single prediction market with:
//! - Bonding curve-based pricing for automatic price discovery
//! - Support for Binary and Multiple Choice markets
//! - Winner-take-all economics where winners receive losers' stakes
//! - Slippage protection on all trades

use odra::prelude::*;
use odra::casper_types::{U256, U512};

use crate::errors::Error;
use crate::types::{
    BondingCurveParams, MarketInfo, MarketStatus, MarketType, UserPosition,
    MarketMetadata, MarketConfig,
};
use crate::events::{
    SharesPurchased, SharesSold, MarketResolved, MarketClosed, MarketCancelled,
    WinningsClaimed, RefundClaimed, ResolverUpdated,
};

/// Convert U512 to U256 (assumes value fits)
fn u512_to_u256(value: U512) -> U256 {
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

/// A prediction market with bonding curve pricing.
#[odra::module(
    errors = Error,
    events = [
        SharesPurchased, SharesSold, MarketResolved, MarketClosed,
        MarketCancelled, WinningsClaimed, RefundClaimed, ResolverUpdated
    ]
)]
pub struct Market {
    // =========================================================================
    // Core Data (Consolidated means < 15 fields)
    // =========================================================================
    /// Encapsulated metadata (description, creator, timestamps, etc.)
    metadata: Var<MarketMetadata>,
    /// Encapsulated configuration (admin, resolver, fees, etc.)
    config: Var<MarketConfig>,
    /// Whether the contract has been initialized
    initialized: Var<bool>,

    // =========================================================================
    // Market State
    // =========================================================================
    /// Current market status
    status: Var<MarketStatus>,
    /// Winning outcome (only set after resolution)
    winning_outcome: Var<u64>,
    /// Total CSPR in the market (across all outcomes)
    total_liquidity: Var<U256>,

    // =========================================================================
    // Bonding Curve State
    // =========================================================================
    /// Bonding curve parameters
    bonding_params: Var<BondingCurveParams>,
    /// Total shares issued per outcome (outcome_id -> total_shares)
    outcome_shares: Mapping<u64, U256>,
    /// Total CSPR invested per outcome (outcome_id -> total_cspr)
    outcome_liquidity: Mapping<u64, U256>,

    // =========================================================================
    // User Positions
    // =========================================================================
    /// User positions: (user_address, outcome_id) -> UserPosition
    user_positions: Mapping<(Address, u64), UserPosition>,
    /// Track if a user has participated (for enumeration purposes)
    user_participated: Mapping<Address, bool>,
    /// Track if user claimed (for resolved/cancelled markets)
    user_claimed: Mapping<Address, bool>,
}

/// Initialization arguments for a Market.
#[odra::odra_type]
pub struct MarketInitArgs {
    pub market_id: u64,
    pub market_type: MarketType,
    pub question: String,
    pub outcome_names: Vec<String>,
    pub end_time: u64,
    pub resolution_source: String,
    pub category: String,
    pub creator: Address,
    pub admin: Address,
    pub resolver: Address,
    pub platform_fee_bps: u64,
    pub vault_contract: Address,
    pub factory_contract: Address,
    pub bonding_params: BondingCurveParams,
}

#[odra::module]
impl Market {
    /// Initialize a new prediction market.
    pub fn init(&mut self, args: MarketInitArgs) {
        if self.initialized.get_or_default() {
            self.env().revert(Error::AlreadyInitialized);
        }

        // Validate outcome count
        let outcome_count = args.outcome_names.len() as u64;
        match args.market_type {
            MarketType::Binary => {
                if outcome_count != 2 {
                    self.env().revert(Error::InvalidOutcomeCount);
                }
            }
            MarketType::MultipleChoice => {
                if outcome_count < 2 {
                    self.env().revert(Error::InvalidOutcomeCount);
                }
            }
            MarketType::Compound => {
                // Compound markets can have any number of sub-predictions
            }
        }

        // Initialize Metadata
        let metadata = MarketMetadata {
            market_id: args.market_id,
            market_type: args.market_type,
            question: args.question,
            outcome_names: args.outcome_names,
            outcome_count,
            creator: args.creator,
            created_at: self.env().get_block_time(),
            end_time: args.end_time,
            resolution_source: args.resolution_source,
            category: args.category,
        };
        self.metadata.set(metadata);

        // Initialize Config
        let config = MarketConfig {
            admin: args.admin,
            resolver: args.resolver,
            platform_fee_bps: args.platform_fee_bps,
            vault_contract: args.vault_contract,
            factory_contract: args.factory_contract,
        };
        self.config.set(config);

        // Initialize bonding curve state
        self.bonding_params.set(args.bonding_params);
        for i in 0..outcome_count {
            self.outcome_shares.set(&i, U256::zero());
            self.outcome_liquidity.set(&i, U256::zero());
        }

        // Set state
        self.status.set(MarketStatus::Active);
        self.total_liquidity.set(U256::zero());
        self.initialized.set(true);
    }

    // =========================================================================
    // Trading Functions
    // =========================================================================

    /// Buy shares in a specific outcome.
    ///
    /// The caller must attach CSPR to this transaction.
    /// Slippage protection via min_shares parameter.
    #[odra(payable)]
    pub fn buy_shares(&mut self, outcome_id: u64, min_shares: U256) {
        self.require_active();
        self.require_not_ended();

        let caller = self.env().caller();
        let attached_value_u512 = self.env().attached_value();
        let attached_value = u512_to_u256(attached_value_u512);

        if attached_value.is_zero() {
            self.env().revert(Error::ZeroAmount);
        }

        // Validate outcome
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        if outcome_id >= metadata.outcome_count {
            self.env().revert(Error::InvalidOutcome);
        }

        // Calculate shares from bonding curve
        let bonding_params = self.bonding_params.get_or_default();
        let current_supply = self.outcome_shares.get(&outcome_id).unwrap_or_default();
        
        // Binary search for the number of shares we can buy with attached_value
        let shares = self.calculate_shares_for_cost(&bonding_params, current_supply, attached_value);
        
        if shares.is_zero() {
            self.env().revert(Error::InsufficientFunds);
        }

        // Check slippage
        if shares < min_shares {
            self.env().revert(Error::SlippageExceeded);
        }

        // Calculate actual cost
        let actual_cost = bonding_params.cost_to_buy(current_supply, shares);
        
        // Refund excess if any
        let refund = attached_value.saturating_sub(actual_cost);

        // Update outcome state
        let new_supply = current_supply.saturating_add(shares);
        self.outcome_shares.set(&outcome_id, new_supply);
        
        let current_liquidity = self.outcome_liquidity.get(&outcome_id).unwrap_or_default();
        self.outcome_liquidity.set(&outcome_id, current_liquidity.saturating_add(actual_cost));

        // Update total liquidity
        let total = self.total_liquidity.get_or_default();
        self.total_liquidity.set(total.saturating_add(actual_cost));

        // Update user position
        let mut position = self.user_positions
            .get(&(caller, outcome_id))
            .unwrap_or_else(|| UserPosition::new(outcome_id));
        position.add_shares(shares, actual_cost);
        self.user_positions.set(&(caller, outcome_id), position);
        self.user_participated.set(&caller, true);

        // Calculate new price for event
        let new_price = bonding_params.price_at_supply(new_supply);

        // Emit event
        self.env().emit_event(SharesPurchased {
            user: caller,
            market_id: metadata.market_id,
            outcome_id,
            shares,
            cost: actual_cost,
            new_price,
            timestamp: self.env().get_block_time(),
        });

        // Refund excess CSPR if any
        if !refund.is_zero() {
            let refund_u512 = u256_to_u512(refund);
            self.env().transfer_tokens(&caller, &refund_u512);
        }
    }

    /// Sell shares in a specific outcome.
    ///
    /// The caller receives CSPR based on the bonding curve minus fees.
    #[odra(non_reentrant)]
    pub fn sell_shares(&mut self, outcome_id: u64, shares: U256, min_receive: U256) {
        self.require_active();
        self.require_not_ended();

        let caller = self.env().caller();

        if shares.is_zero() {
            self.env().revert(Error::ZeroAmount);
        }

        // Validate outcome
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        if outcome_id >= metadata.outcome_count {
            self.env().revert(Error::InvalidOutcome);
        }

        // Check user has enough shares
        let mut position = self.user_positions
            .get(&(caller, outcome_id))
            .unwrap_or_else(|| UserPosition::new(outcome_id));
        
        if position.shares < shares {
            self.env().revert(Error::InsufficientShares);
        }

        // Calculate revenue from bonding curve
        let bonding_params = self.bonding_params.get_or_default();
        let current_supply = self.outcome_shares.get(&outcome_id).unwrap_or_default();
        let gross_revenue = bonding_params.revenue_from_sell(current_supply, shares);

        // Deduct platform fee
        let config = self.config.get().unwrap_or_revert(&self.env());
        let fee = gross_revenue.saturating_mul(U256::from(config.platform_fee_bps)) / U256::from(10_000u64);
        let net_revenue = gross_revenue.saturating_sub(fee);

        // Check slippage
        if net_revenue < min_receive {
            self.env().revert(Error::SlippageExceeded);
        }

        // Update outcome state
        let new_supply = current_supply.saturating_sub(shares);
        self.outcome_shares.set(&outcome_id, new_supply);

        let current_liquidity = self.outcome_liquidity.get(&outcome_id).unwrap_or_default();
        self.outcome_liquidity.set(&outcome_id, current_liquidity.saturating_sub(gross_revenue));

        // Update total liquidity (fees stay in market for now)
        let total = self.total_liquidity.get_or_default();
        self.total_liquidity.set(total.saturating_sub(net_revenue));

        // Update user position
        position.remove_shares(shares, net_revenue);
        self.user_positions.set(&(caller, outcome_id), position);

        // Calculate new price for event
        let new_price = bonding_params.price_at_supply(new_supply);

        // Emit event
        self.env().emit_event(SharesSold {
            user: caller,
            market_id: metadata.market_id,
            outcome_id,
            shares,
            revenue: net_revenue,
            new_price,
            timestamp: self.env().get_block_time(),
        });

        // Transfer CSPR to seller
        let net_revenue_u512 = u256_to_u512(net_revenue);
        self.env().transfer_tokens(&caller, &net_revenue_u512);
    }

    // =========================================================================
    // Resolution Functions
    // =========================================================================

    /// Resolve the market with the winning outcome.
    ///
    /// Can only be called by the designated resolver or admin.
    pub fn resolve_market(&mut self, winning_outcome_id: u64, proof: String) {
        self.require_closed_or_ended();
        
        let caller = self.env().caller();
        let config = self.config.get().unwrap_or_revert(&self.env());
        
        if caller != config.resolver && caller != config.admin {
            self.env().revert(Error::NotResolver);
        }

        // Check not already resolved
        let status = self.status.get_or_default();
        if matches!(status, MarketStatus::Resolved) {
            self.env().revert(Error::MarketAlreadyResolved);
        }

        // Validate outcome
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        if winning_outcome_id >= metadata.outcome_count {
            self.env().revert(Error::InvalidOutcome);
        }

        // Set winning outcome and status
        self.winning_outcome.set(winning_outcome_id);
        self.status.set(MarketStatus::Resolved);

        self.env().emit_event(MarketResolved {
            market_id: metadata.market_id,
            winning_outcome: winning_outcome_id,
            resolver: caller,
            timestamp: self.env().get_block_time(),
            proof,
        });
    }

    /// Claim winnings from a resolved market.
    ///
    /// Winners receive their proportional share of the total pool.
    #[odra(non_reentrant)]
    pub fn claim_winnings(&mut self) {
        let status = self.status.get_or_default();
        if !matches!(status, MarketStatus::Resolved) {
            self.env().revert(Error::MarketNotResolved);
        }

        let caller = self.env().caller();

        // Check not already claimed
        if self.user_claimed.get(&caller).unwrap_or(false) {
            self.env().revert(Error::AlreadyClaimed);
        }

        let winning_outcome_id = self.winning_outcome.get_or_default();

        // Get user's winning position
        let position = self.user_positions
            .get(&(caller, winning_outcome_id))
            .unwrap_or_else(|| UserPosition::new(winning_outcome_id));

        if position.shares.is_zero() {
            self.env().revert(Error::NoWinningPosition);
        }

        // Calculate payout
        // Winner gets: (user_shares / total_winning_shares) * total_pool
        let total_winning_shares = self.outcome_shares.get(&winning_outcome_id).unwrap_or_default();
        let total_pool = self.total_liquidity.get_or_default();

        // Deduct platform fee from total pool
        let config = self.config.get().unwrap_or_revert(&self.env());
        let total_fee = total_pool.saturating_mul(U256::from(config.platform_fee_bps)) / U256::from(10_000u64);
        let distributable_pool = total_pool.saturating_sub(total_fee);

        // Calculate user's share
        let payout = if total_winning_shares.is_zero() {
            U256::zero()
        } else {
            distributable_pool.saturating_mul(position.shares) / total_winning_shares
        };

        if payout.is_zero() {
            self.env().revert(Error::NothingToClaim);
        }

        // Mark as claimed
        self.user_claimed.set(&caller, true);
        
        // Emit event needs market_id
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        self.env().emit_event(WinningsClaimed {
            user: caller,
            market_id: metadata.market_id,
            payout,
            timestamp: self.env().get_block_time(),
        });

        // Transfer payout
        let payout_u512 = u256_to_u512(payout);
        self.env().transfer_tokens(&caller, &payout_u512);
    }

    /// Claim refund from a cancelled market.
    #[odra(non_reentrant)]
    pub fn claim_refund(&mut self) {
        let status = self.status.get_or_default();
        if !matches!(status, MarketStatus::Cancelled) {
            self.env().revert(Error::MarketNotCancelled);
        }

        let caller = self.env().caller();

        // Check not already claimed
        if self.user_claimed.get(&caller).unwrap_or(false) {
            self.env().revert(Error::AlreadyClaimed);
        }

        // Calculate total cost basis across all outcomes
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        let mut total_refund = U256::zero();

        for outcome_id in 0..metadata.outcome_count {
            let position = self.user_positions
                .get(&(caller, outcome_id))
                .unwrap_or_else(|| UserPosition::new(outcome_id));
            total_refund = total_refund.saturating_add(position.total_cost);
        }

        if total_refund.is_zero() {
            self.env().revert(Error::NoPositionToRefund);
        }

        // Mark as claimed
        self.user_claimed.set(&caller, true);

        self.env().emit_event(RefundClaimed {
            user: caller,
            market_id: metadata.market_id,
            amount: total_refund,
            timestamp: self.env().get_block_time(),
        });

        // Transfer refund
        let refund_u512 = u256_to_u512(total_refund);
        self.env().transfer_tokens(&caller, &refund_u512);
    }

    // =========================================================================
    // Admin Functions
    // =========================================================================

    /// Close the market for trading.
    pub fn close_market(&mut self) {
        self.require_admin();
        
        let status = self.status.get_or_default();
        if !matches!(status, MarketStatus::Active) {
            self.env().revert(Error::MarketNotActive);
        }

        self.status.set(MarketStatus::Closed);
        
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        self.env().emit_event(MarketClosed {
            market_id: metadata.market_id,
            timestamp: self.env().get_block_time(),
        });
    }

    /// Cancel the market and enable refunds.
    pub fn cancel_market(&mut self, reason: String) {
        self.require_admin();

        let status = self.status.get_or_default();
        if matches!(status, MarketStatus::Resolved) {
            self.env().revert(Error::MarketAlreadyResolved);
        }

        self.status.set(MarketStatus::Cancelled);
        
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        self.env().emit_event(MarketCancelled {
            market_id: metadata.market_id,
            reason,
            timestamp: self.env().get_block_time(),
        });
    }

    /// Update the resolver address.
    pub fn update_resolver(&mut self, new_resolver: Address) {
        self.require_admin();
        
        let mut config = self.config.get().unwrap_or_revert(&self.env());
        let previous_resolver = config.resolver;
        config.resolver = new_resolver;
        self.config.set(config);
        
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        self.env().emit_event(ResolverUpdated {
            market_id: metadata.market_id,
            previous_resolver,
            new_resolver,
        });
    }

    // =========================================================================
    // View Functions
    // =========================================================================

    /// Get the current price for an outcome.
    pub fn get_current_price(&self, outcome_id: u64) -> U256 {
        let bonding_params = self.bonding_params.get_or_default();
        let current_supply = self.outcome_shares.get(&outcome_id).unwrap_or_default();
        bonding_params.price_at_supply(current_supply)
    }

    /// Calculate the cost to buy a specific number of shares.
    pub fn calculate_buy_cost(&self, outcome_id: u64, shares: U256) -> U256 {
        let bonding_params = self.bonding_params.get_or_default();
        let current_supply = self.outcome_shares.get(&outcome_id).unwrap_or_default();
        bonding_params.cost_to_buy(current_supply, shares)
    }

    /// Calculate the revenue from selling a specific number of shares.
    pub fn calculate_sell_revenue(&self, outcome_id: u64, shares: U256) -> U256 {
        let bonding_params = self.bonding_params.get_or_default();
        let config = self.config.get().unwrap_or_revert(&self.env());
        let current_supply = self.outcome_shares.get(&outcome_id).unwrap_or_default();
        
        let gross = bonding_params.revenue_from_sell(current_supply, shares);
        let fee = gross.saturating_mul(U256::from(config.platform_fee_bps)) / U256::from(10_000u64);
        gross.saturating_sub(fee)
    }

    /// Get complete market information.
    pub fn get_market_info(&self) -> MarketInfo {
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        
        MarketInfo {
            market_id: metadata.market_id,
            market_type: metadata.market_type,
            question: metadata.question,
            creator: metadata.creator,
            end_time: metadata.end_time,
            status: self.status.get_or_default(),
            total_liquidity: self.total_liquidity.get_or_default(),
            category: metadata.category,
            resolution_source: metadata.resolution_source,
        }
    }

    /// Get a user's position for a specific outcome.
    pub fn get_user_position(&self, user: Address, outcome_id: u64) -> UserPosition {
        self.user_positions
            .get(&(user, outcome_id))
            .unwrap_or_else(|| UserPosition::new(outcome_id))
    }

    /// Get total shares for an outcome.
    pub fn get_outcome_shares(&self, outcome_id: u64) -> U256 {
        self.outcome_shares.get(&outcome_id).unwrap_or_default()
    }

    /// Get total liquidity for an outcome.
    pub fn get_outcome_liquidity(&self, outcome_id: u64) -> U256 {
        self.outcome_liquidity.get(&outcome_id).unwrap_or_default()
    }

    /// Get the number of outcomes.
    pub fn get_outcome_count(&self) -> u64 {
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        metadata.outcome_count
    }

    /// Get outcome name by ID.
    pub fn get_outcome_name(&self, outcome_id: u64) -> Option<String> {
        let metadata = self.metadata.get()?;
        if (outcome_id as usize) < metadata.outcome_names.len() {
            Some(metadata.outcome_names.get(outcome_id as usize).unwrap_or(&String::default()).clone())
        } else {
            None
        }
    }

    /// Get implied odds for all outcomes (as percentage * 100).
    /// Returns a vector of (outcome_id, odds_percentage).
    pub fn get_outcome_odds(&self) -> Vec<(u64, u64)> {
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        let outcome_count = metadata.outcome_count;
        let mut total_inverse_price = U256::zero();
        let mut prices = Vec::new();

        let bonding_params = self.bonding_params.get_or_default();

        // Calculate current prices
        for i in 0..outcome_count {
            let supply = self.outcome_shares.get(&i).unwrap_or_default();
            let price = bonding_params.price_at_supply(supply);
            prices.push((i, price));
            if !price.is_zero() {
                // Use a large multiplier for precision
                total_inverse_price = total_inverse_price.saturating_add(
                    U256::from(1_000_000_000_000u64) / price
                );
            }
        }

        // Calculate normalized odds
        let mut odds = Vec::new();
        for (outcome_id, price) in prices {
            if price.is_zero() || total_inverse_price.is_zero() {
                odds.push((outcome_id, 0u64));
            } else {
                let inverse = U256::from(1_000_000_000_000u64) / price;
                let probability = inverse.saturating_mul(U256::from(10000u64)) / total_inverse_price;
                odds.push((outcome_id, probability.as_u64()));
            }
        }

        odds
    }

    /// Get the winning outcome (only valid after resolution).
    pub fn get_winning_outcome(&self) -> Option<u64> {
        let status = self.status.get_or_default();
        if matches!(status, MarketStatus::Resolved) {
            Some(self.winning_outcome.get_or_default())
        } else {
            None
        }
    }

    /// Check if the market is past its end time.
    pub fn is_ended(&self) -> bool {
        let metadata = self.metadata.get().unwrap_or_revert(&self.env());
        self.env().get_block_time() >= metadata.end_time
    }

    /// Get the market status.
    pub fn get_status(&self) -> MarketStatus {
        self.status.get_or_default()
    }

    /// Get metadata
    pub fn get_metadata(&self) -> MarketMetadata {
        self.metadata.get().unwrap_or_revert(&self.env())
    }
    
    /// Get config
    pub fn get_config(&self) -> MarketConfig {
        self.config.get().unwrap_or_revert(&self.env())
    }

    // =========================================================================
    // Internal Helper Functions
    // =========================================================================

    /// Ensure market is in Active status.
    fn require_active(&self) {
        let status = self.status.get_or_default();
        if !matches!(status, MarketStatus::Active) {
            self.env().revert(Error::MarketNotActive);
        }
    }

    /// Ensure market hasn't ended.
    fn require_not_ended(&self) {
        if self.is_ended() {
            self.env().revert(Error::MarketAlreadyEnded);
        }
    }

    /// Ensure market is closed or has ended.
    fn require_closed_or_ended(&self) {
        let status = self.status.get_or_default();
        let is_closed = matches!(status, MarketStatus::Closed);
        let is_ended = self.is_ended();
        
        if !is_closed && !is_ended {
            self.env().revert(Error::MarketNotEnded);
        }
    }

    /// Ensure caller is admin.
    fn require_admin(&self) {
        let caller = self.env().caller();
        let config = self.config.get().unwrap_or_revert(&self.env());
        if caller != config.admin {
            self.env().revert(Error::NotAdmin);
        }
    }

    /// Binary search to find how many shares can be bought for a given cost.
    fn calculate_shares_for_cost(
        &self,
        params: &BondingCurveParams,
        current_supply: U256,
        max_cost: U256,
    ) -> U256 {
        if max_cost.is_zero() {
            return U256::zero();
        }

        // Binary search for the maximum number of shares
        let mut low = U256::zero();
        let mut high = max_cost / params.initial_price; // Upper bound estimate
        high = high.saturating_add(U256::from(1000u64)); // Add buffer

        while low < high {
            let mid = (low + high + U256::one()) / U256::from(2u64);
            let cost = params.cost_to_buy(current_supply, mid);
            
            if cost <= max_cost {
                low = mid;
            } else {
                high = mid - U256::one();
            }
        }

        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, HostRef};

    fn setup_binary_market() -> (MarketHostRef, odra::host::HostEnv) {
        let env = odra_test::env();
        let admin = env.get_account(0);
        
        let init_args = MarketInitArgs {
            market_id: 1,
            market_type: MarketType::Binary,
            question: "Will BTC reach $100k?".to_string(),
            outcome_names: vec!["Yes".to_string(), "No".to_string()],
            end_time: env.get_block_time() + 86400, // 1 day from now
            resolution_source: "Price oracle".to_string(),
            category: "crypto".to_string(),
            creator: admin,
            admin,
            resolver: admin,
            platform_fee_bps: 200, // 2%
            vault_contract: admin, // Placeholder
            factory_contract: admin, // Placeholder
            bonding_params: BondingCurveParams::default_params(),
        };

        let market = Market::deploy(&env, init_args);
        (market, env)
    }

    #[test]
    fn test_market_init() {
        let (market, _env) = setup_binary_market();
        
        let info = market.get_market_info();
        assert_eq!(info.market_id, 1);
        assert_eq!(info.question, "Will BTC reach $100k?");
        assert!(matches!(info.status, MarketStatus::Active));
        assert_eq!(market.get_outcome_count(), 2);
    }

    #[test]
    fn test_get_outcome_names() {
        let (market, _env) = setup_binary_market();
        
        assert_eq!(market.get_outcome_name(0), Some("Yes".to_string()));
        assert_eq!(market.get_outcome_name(1), Some("No".to_string()));
        assert_eq!(market.get_outcome_name(2), None);
    }

    #[test]
    fn test_initial_price() {
        let (market, _env) = setup_binary_market();
        
        // Initial price should be the base price
        let price = market.get_current_price(0);
        assert_eq!(price, U256::from(10_000_000u64)); // 0.01 CSPR
    }
}
