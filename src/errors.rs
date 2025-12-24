//! Custom error types for Casper Predict contracts.
//!
//! All errors have unique discriminant values to ensure
//! they can be properly identified when transactions revert.

use odra::prelude::*;

/// Errors that can occur in the Casper Predict contracts.
#[odra::odra_error]
pub enum Error {
    // =========================================================================
    // Access Control Errors (1-9)
    // =========================================================================
    /// Caller is not authorized to perform this action
    AccessDenied = 1,
    /// Caller is not the contract admin
    NotAdmin = 2,
    /// Caller is not the designated resolver
    NotResolver = 3,
    /// Caller is not an authorized market contract
    NotAuthorizedMarket = 4,
    /// Caller is not the factory contract
    NotFactory = 5,

    // =========================================================================
    // Market State Errors (10-19)
    // =========================================================================
    /// Market is not in the Active state
    MarketNotActive = 10,
    /// Market is not in the Closed state
    MarketNotClosed = 11,
    /// Market is not in the Resolved state
    MarketNotResolved = 12,
    /// Market has already been resolved
    MarketAlreadyResolved = 13,
    /// Market end time has not been reached yet
    MarketNotEnded = 14,
    /// Market has been cancelled
    MarketCancelled = 15,
    /// Market end time has already passed
    MarketAlreadyEnded = 16,
    /// Market is not in the Cancelled state
    MarketNotCancelled = 17,

    // =========================================================================
    // Trading Errors (20-29)
    // =========================================================================
    /// User does not have enough shares to sell
    InsufficientShares = 20,
    /// Attached CSPR is insufficient for the purchase
    InsufficientFunds = 21,
    /// Price slippage exceeds the specified limit
    SlippageExceeded = 22,
    /// The specified outcome ID is invalid
    InvalidOutcome = 23,
    /// Cannot trade zero shares
    ZeroAmount = 24,
    /// Share amount must be positive
    InvalidShareAmount = 25,
    /// Minimum receive amount not met
    MinimumNotMet = 26,

    // =========================================================================
    // Vault Errors (30-39)
    // =========================================================================
    /// Market contract is not authorized to interact with vault
    UnauthorizedMarket = 30,
    /// Vault does not have sufficient balance for this operation
    InsufficientVaultBalance = 31,
    /// Vault operations are currently paused
    VaultPaused = 32,
    /// Market has already been authorized
    MarketAlreadyAuthorized = 33,
    /// Market is not authorized
    MarketNotAuthorized = 34,
    /// Cannot withdraw more than market balance
    ExceedsMarketBalance = 35,

    // =========================================================================
    // Factory Errors (40-49)
    // =========================================================================
    /// Market creation is currently paused
    FactoryPaused = 40,
    /// Market duration is outside the allowed range
    InvalidMarketDuration = 41,
    /// Initial liquidity is below the minimum required
    InsufficientInitialLiquidity = 42,
    /// Invalid number of outcomes for market type
    InvalidOutcomeCount = 43,
    /// Question string is empty or too long
    InvalidQuestion = 44,
    /// Vault contract has not been set
    VaultNotSet = 45,
    /// Fee percentage is too high
    FeeTooHigh = 46,

    // =========================================================================
    // Claim Errors (50-59)
    // =========================================================================
    /// User has already claimed their winnings/refund
    AlreadyClaimed = 50,
    /// User has nothing to claim
    NothingToClaim = 51,
    /// User does not have a winning position
    NoWinningPosition = 52,
    /// User does not have any position to refund
    NoPositionToRefund = 53,

    // =========================================================================
    // Math/Overflow Errors (60-69)
    // =========================================================================
    /// Arithmetic overflow occurred
    Overflow = 60,
    /// Arithmetic underflow occurred
    Underflow = 61,
    /// Division by zero
    DivisionByZero = 62,

    // =========================================================================
    // Initialization Errors (70-79)
    // =========================================================================
    /// Contract has already been initialized
    AlreadyInitialized = 70,
    /// Contract has not been initialized
    NotInitialized = 71,
    /// Invalid initialization parameters
    InvalidInitParams = 72,
}
