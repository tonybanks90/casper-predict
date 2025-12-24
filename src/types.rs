//! Core types shared across all Casper Predict contracts.
//!
//! This module defines the fundamental data structures used by the prediction
//! market system including market types, statuses, outcomes, and user positions.

use odra::prelude::*;
use odra::casper_types::U256;
use odra::prelude::Address;

/// The type of prediction market.
#[odra::odra_type]
#[derive(Default)]
pub enum MarketType {
    /// Binary market with YES/NO outcomes
    #[default]
    Binary,
    /// Multiple choice market with 3+ outcomes
    MultipleChoice,
    /// Compound market with multiple related sub-predictions
    Compound,
}

/// The current state of a prediction market.
#[odra::odra_type]
#[derive(Default)]
pub enum MarketStatus {
    /// Trading is open
    #[default]
    Active,
    /// Trading is closed, awaiting resolution
    Closed,
    /// Outcome has been determined, payouts available
    Resolved,
    /// Market has been cancelled, refunds available
    Cancelled,
}

/// Represents a single outcome option in a prediction market.
#[odra::odra_type]
pub struct Outcome {
    /// Unique identifier for this outcome within the market
    pub id: u64,
    /// Human-readable name for this outcome
    pub name: String,
    /// Total number of shares issued for this outcome
    pub total_shares: U256,
}

impl Outcome {
    /// Creates a new outcome with the given id and name.
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            total_shares: U256::zero(),
        }
    }
}

/// Represents a user's position in a specific market outcome.
#[odra::odra_type]
#[derive(Default)]
pub struct UserPosition {
    /// The outcome this position is for
    pub outcome_id: u64,
    /// Number of shares held
    pub shares: U256,
    /// Total CSPR spent to acquire these shares
    pub total_cost: U256,
    /// Whether winnings have been claimed (for resolved markets)
    pub claimed: bool,
}

impl UserPosition {
    /// Creates a new empty position for the given outcome.
    pub fn new(outcome_id: u64) -> Self {
        Self {
            outcome_id,
            shares: U256::zero(),
            total_cost: U256::zero(),
            claimed: false,
        }
    }

    /// Adds shares to this position.
    pub fn add_shares(&mut self, shares: U256, cost: U256) {
        self.shares = self.shares.saturating_add(shares);
        self.total_cost = self.total_cost.saturating_add(cost);
    }

    /// Removes shares from this position.
    /// Returns true if successful, false if insufficient shares.
    pub fn remove_shares(&mut self, shares: U256, _revenue: U256) -> bool {
        if self.shares < shares {
            return false;
        }
        self.shares = self.shares.saturating_sub(shares);
        // Proportionally reduce cost basis
        if self.shares.is_zero() {
            self.total_cost = U256::zero();
        } else {
            let cost_reduction = self.total_cost.saturating_mul(shares) / (self.shares + shares);
            self.total_cost = self.total_cost.saturating_sub(cost_reduction);
        }
        true
    }
}

/// Comprehensive market information returned by view functions.
#[odra::odra_type]
pub struct MarketInfo {
    /// Unique market identifier
    pub market_id: u64,
    /// Type of market (Binary, MultipleChoice, Compound)
    pub market_type: MarketType,
    /// The prediction question
    pub question: String,
    /// Address of the market creator
    pub creator: Address,
    /// Unix timestamp when trading ends
    pub end_time: u64,
    /// Current status of the market
    pub status: MarketStatus,
    /// Total CSPR liquidity in the market
    pub total_liquidity: U256,
    /// Category of the market (e.g., "crypto", "sports")
    pub category: String,
    /// Resolution data source
    pub resolution_source: String,
}

/// Consolidated metadata for a market to reduce contract fields.
#[odra::odra_type]
pub struct MarketMetadata {
    /// Unique market identifier
    pub market_id: u64,
    /// Type of market (Binary, MultipleChoice, Compound)
    pub market_type: MarketType,
    /// The prediction question
    pub question: String,
    /// List of outcome names
    pub outcome_names: Vec<String>,
    /// Number of outcomes
    pub outcome_count: u64,
    /// Address of the market creator
    pub creator: Address,
    /// Unix timestamp of creation
    pub created_at: u64,
    /// Unix timestamp when trading ends
    pub end_time: u64,
    /// Resolution data source
    pub resolution_source: String,
    /// Category of the market
    pub category: String,
}

/// Consolidated configuration for a market.
#[odra::odra_type]
pub struct MarketConfig {
    /// Contract admin
    pub admin: Address,
    /// Designated resolver for this market
    pub resolver: Address,
    /// Platform fee in basis points
    pub platform_fee_bps: u64,
    /// Vault contract address
    pub vault_contract: Address,
    /// Factory contract address
    pub factory_contract: Address,
}

/// Parameters for creating a new binary market.
#[odra::odra_type]
pub struct BinaryMarketParams {
    /// The prediction question
    pub question: String,
    /// Unix timestamp when trading ends
    pub end_time: u64,
    /// Data source for resolution
    pub resolution_source: String,
    /// Market category
    pub category: String,
}

/// Parameters for creating a new multiple choice market.
#[odra::odra_type]
pub struct MultipleChoiceMarketParams {
    /// The prediction question
    pub question: String,
    /// List of outcome names
    pub outcomes: Vec<String>,
    /// Unix timestamp when trading ends
    pub end_time: u64,
    /// Data source for resolution
    pub resolution_source: String,
    /// Market category
    pub category: String,
}

/// Bonding curve configuration parameters.
#[odra::odra_type]
#[derive(Default)]
pub struct BondingCurveParams {
    /// Base price per share in motes (1 CSPR = 1_000_000_000 motes)
    /// Default: 10_000_000 motes = 0.01 CSPR
    pub initial_price: U256,
    /// Curve steepness constant (scaled by 1e9 for precision)
    /// Default: 1_000_000 = 0.001 CSPR per share
    pub k_constant: U256,
}

impl BondingCurveParams {
    /// Creates default bonding curve parameters.
    /// Initial price: 0.01 CSPR, K: 0.001 CSPR per share
    pub fn default_params() -> Self {
        Self {
            initial_price: U256::from(10_000_000u64), // 0.01 CSPR in motes
            k_constant: U256::from(1_000_000u64),     // 0.001 CSPR per share
        }
    }

    /// Calculate the price for a specific supply level.
    /// price(supply) = initial_price + (k * supply)
    pub fn price_at_supply(&self, supply: U256) -> U256 {
        self.initial_price.saturating_add(self.k_constant.saturating_mul(supply))
    }

    /// Calculate the cost to buy n shares starting from current_supply.
    /// cost = n * initial_price + k * [n*(2*current_supply + n + 1)/2]
    pub fn cost_to_buy(&self, current_supply: U256, shares: U256) -> U256 {
        if shares.is_zero() {
            return U256::zero();
        }

        // Base cost: n * initial_price
        let base_cost = shares.saturating_mul(self.initial_price);

        // Premium cost from bonding curve:
        // Sum of (k * i) for i from current_supply+1 to current_supply+n
        // = k * sum(i) = k * [n * (2*current_supply + n + 1) / 2]
        let two = U256::from(2u64);
        let sum_factor = shares.saturating_mul(
            two.saturating_mul(current_supply)
                .saturating_add(shares)
                .saturating_add(U256::one())
        ) / two;
        let premium = self.k_constant.saturating_mul(sum_factor);

        base_cost.saturating_add(premium)
    }

    /// Calculate the revenue from selling n shares starting from current_supply.
    /// This is the inverse of cost_to_buy, from current_supply-n to current_supply.
    pub fn revenue_from_sell(&self, current_supply: U256, shares: U256) -> U256 {
        if shares.is_zero() || current_supply < shares {
            return U256::zero();
        }

        let new_supply = current_supply.saturating_sub(shares);
        self.cost_to_buy(new_supply, shares)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bonding_curve_price_increases() {
        let params = BondingCurveParams::default_params();
        
        let price_at_0 = params.price_at_supply(U256::zero());
        let price_at_100 = params.price_at_supply(U256::from(100u64));
        let price_at_1000 = params.price_at_supply(U256::from(1000u64));
        
        assert!(price_at_100 > price_at_0);
        assert!(price_at_1000 > price_at_100);
    }

    #[test]
    fn test_bonding_curve_cost_calculation() {
        let params = BondingCurveParams::default_params();
        
        // Cost to buy first share should be initial_price + k
        let cost_first = params.cost_to_buy(U256::zero(), U256::one());
        // = 1 * initial_price + k * [1 * (0 + 1 + 1) / 2] = initial_price + k
        let expected = params.initial_price.saturating_add(params.k_constant);
        assert_eq!(cost_first, expected);
    }

    #[test]
    fn test_user_position_add_remove() {
        let mut pos = UserPosition::new(0);
        
        pos.add_shares(U256::from(100u64), U256::from(1000u64));
        assert_eq!(pos.shares, U256::from(100u64));
        assert_eq!(pos.total_cost, U256::from(1000u64));
        
        let success = pos.remove_shares(U256::from(50u64), U256::from(600u64));
        assert!(success);
        assert_eq!(pos.shares, U256::from(50u64));
    }
}
