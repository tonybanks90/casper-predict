//! Casper Predict - Decentralized Prediction Marketplace
//!
//! This crate contains the smart contracts for Casper Predict, a prediction
//! marketplace built on the Casper Network featuring:
//!
//! - **Bonding Curve Pricing**: Automatic price discovery without liquidity pools
//! - **Winner-Take-All Economics**: Winners receive losers' stakes
//! - **Early Adopter Advantage**: First buyers get better prices
//!
//! ## Contract Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │                 MarketFactory                        │
//! │  - Creates all market types                         │
//! │  - Manages market registry                          │
//! │  - Deploys Market contracts                         │
//! └──────────────┬──────────────────────────────────────┘
//!                │
//!                │ Creates
//!                │
//!                ├──────────────────────────────────────┐
//!                │                                      │
//!                ▼                                      ▼
//! ┌──────────────────────────┐         ┌────────────────────────────┐
//! │    Market Contract       │◄────────┤    Vault Contract          │
//! │  - Bonding curve logic   │  Holds  │  - Holds all CSPR funds   │
//! │  - Buy/Sell shares       │  Funds  │  - Escrow management      │
//! │  - Resolution logic      │         │  - Payout distribution    │
//! │  - Admin controls        │         │  - Security controls      │
//! └──────────────────────────┘         └────────────────────────────┘
//! ```
//!
//! ## Modules
//!
//! - [`types`] - Core data types and bonding curve math
//! - [`events`] - Event definitions for all contracts
//! - [`errors`] - Custom error types
//! - [`vault`] - Secure CSPR escrow contract
//! - [`market`] - Individual prediction market contract
//! - [`factory`] - Market creation and registry contract

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

pub mod types;
pub mod events;
pub mod errors;
pub mod vault;
pub mod market;
pub mod factory;

// Re-export main contracts for convenience
pub use vault::Vault;
pub use market::Market;
pub use factory::MarketFactory;
