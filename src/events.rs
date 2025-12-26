//! Events emitted by Casper Predict contracts.
//!
//! All events follow the Casper Event Standard (CES) and are designed
//! to be easily consumed by off-chain applications and block explorers.

use odra::prelude::*;
use odra::casper_types::U256;

// =============================================================================
// MarketFactory Events
// =============================================================================

/// Emitted when a new prediction market is created.
#[odra::event]
pub struct MarketCreated {
    /// Unique identifier for the market
    pub market_id: u64,
    /// Type of market (0=Binary, 1=MultipleChoice, 2=Compound)
    pub market_type: u8,
    /// Address of the market creator
    pub creator: Address,
    /// The prediction question
    pub question: String,
    /// Unix timestamp when trading ends
    pub end_time: u64,
    /// Market category
    pub category: String,
}

/// Emitted when the platform fee is updated.
#[odra::event]
pub struct PlatformFeeUpdated {
    /// Previous fee percentage (basis points, e.g., 200 = 2%)
    pub old_fee: u64,
    /// New fee percentage
    pub new_fee: u64,
}

/// Emitted when the factory is paused or unpaused.
#[odra::event]
pub struct FactoryPauseStatusChanged {
    /// Whether the factory is now paused
    pub paused: bool,
}

// =============================================================================
// Market Events
// =============================================================================

/// Emitted when a user purchases shares in a market outcome.
#[odra::event]
pub struct SharesPurchased {
    /// Address of the buyer
    pub user: Address,
    /// Market identifier
    pub market_id: u64,
    /// Outcome being purchased
    pub outcome_id: u64,
    /// Number of shares purchased
    pub shares: U256,
    /// Total CSPR cost
    pub cost: U256,
    /// New price after purchase
    pub new_price: U256,
    /// Block timestamp
    pub timestamp: u64,
}

/// Emitted when a user sells shares in a market outcome.
#[odra::event]
pub struct SharesSold {
    /// Address of the seller
    pub user: Address,
    /// Market identifier
    pub market_id: u64,
    /// Outcome being sold
    pub outcome_id: u64,
    /// Number of shares sold
    pub shares: U256,
    /// CSPR revenue received
    pub revenue: U256,
    /// New price after sale
    pub new_price: U256,
    /// Block timestamp
    pub timestamp: u64,
}

/// Emitted when a market is resolved.
#[odra::event]
pub struct MarketResolved {
    /// Market identifier
    pub market_id: u64,
    /// The winning outcome ID
    pub winning_outcome: u64,
    /// Address of the resolver
    pub resolver: Address,
    /// Block timestamp
    pub timestamp: u64,
    /// Proof or reference to resolution data
    pub proof: String,
}

/// Emitted when a market is closed for trading.
#[odra::event]
pub struct MarketClosed {
    /// Market identifier
    pub market_id: u64,
    /// Block timestamp
    pub timestamp: u64,
}

/// Emitted when a market is cancelled.
#[odra::event]
pub struct MarketCancelled {
    /// Market identifier
    pub market_id: u64,
    /// Reason for cancellation
    pub reason: String,
    /// Block timestamp
    pub timestamp: u64,
}

/// Emitted when a winner claims their payout.
#[odra::event]
pub struct WinningsClaimed {
    /// Address of the claimer
    pub user: Address,
    /// Market identifier
    pub market_id: u64,
    /// Total payout amount in CSPR
    pub payout: U256,
    /// Block timestamp
    pub timestamp: u64,
}

/// Emitted when a user claims a refund from a cancelled market.
#[odra::event]
pub struct RefundClaimed {
    /// Address of the claimer
    pub user: Address,
    /// Market identifier
    pub market_id: u64,
    /// Refund amount in CSPR
    pub amount: U256,
    /// Block timestamp
    pub timestamp: u64,
}

// =============================================================================
// Vault Events
// =============================================================================

/// Emitted when CSPR is deposited into the vault.
#[odra::event]
pub struct FundsDeposited {
    /// Market identifier the funds belong to
    pub market_id: u64,
    /// Amount deposited in motes
    pub amount: U256,
    /// Contract that initiated the deposit
    pub from_contract: Address,
}

/// Emitted when CSPR is withdrawn from the vault.
#[odra::event]
pub struct FundsWithdrawn {
    /// Market identifier the funds belong to
    pub market_id: u64,
    /// Recipient address
    pub recipient: Address,
    /// Amount withdrawn in motes
    pub amount: U256,
}

/// Emitted when platform fees are collected.
#[odra::event]
pub struct FeesCollected {
    /// Market identifier
    pub market_id: u64,
    /// Fee amount collected
    pub amount: U256,
}

/// Emitted when platform fees are claimed by the recipient.
#[odra::event]
pub struct FeesClaimed {
    /// Recipient address
    pub recipient: Address,
    /// Amount claimed
    pub amount: U256,
}

/// Emitted when a market contract is authorized.
#[odra::event]
pub struct MarketAuthorized {
    /// Address of the authorized market contract
    pub market: Address,
}

/// Emitted when a market contract authorization is revoked.
#[odra::event]
pub struct MarketRevoked {
    /// Address of the revoked market contract
    pub market: Address,
}

/// Emitted when the vault is paused or unpaused.
#[odra::event]
pub struct VaultPauseStatusChanged {
    /// Whether the vault is now paused
    pub paused: bool,
}

// =============================================================================
// Admin Events
// =============================================================================

/// Emitted when contract ownership is transferred.
#[odra::event]
pub struct AdminTransferred {
    /// Previous admin address
    pub previous_admin: Address,
    /// New admin address
    pub new_admin: Address,
}

/// Emitted when the resolver for a market is updated.
#[odra::event]
pub struct ResolverUpdated {
    /// Market identifier
    pub market_id: u64,
    /// Previous resolver address
    pub previous_resolver: Address,
    /// New resolver address
    pub new_resolver: Address,
}
