# Casper Predict Deployment Guide

This guide explains how to manually deploy the Casper Predict smart contracts to the Casper Testnet using [cspr.live](https://testnet.cspr.live/).

## Prerequisites

1.  **Casper Wallet**: Install the [Casper Wallet](https://www.casperwallet.io/) browser extension.
2.  **Funded Account**: Ensure your account has sufficient CSPR on Testnet. You can use the [Testnet Faucet](https://testnet.cspr.live/tools/faucet).
3.  **WASM Files**: Locate the compiled contract files in your project directory:
    - `wasm/Vault.wasm`
    - `wasm/MarketFactory.wasm`
    - `wasm/Market.wasm`

## Deployment Steps

### 1. Deploy the Vault Contract

The Vault holds all funds and manages fees.

1.  Go to [cspr.live/deploy](https://testnet.cspr.live/deploy).
2.  Upload `wasm/Vault.wasm`.
3.  **Arguments**:
    - `admin` (Type: `Key`): Enter your account address/hash.
    - `fee_recipient` (Type: `Key`): Enter the account address to receive platform fees (can be same as admin).
4.  Sign and submit the deployment.
5.  Wait for execution. **Copy the Contract Hash** (e.g., `hash-123...`). You will need this.

### 2. Deploy the MarketFactory Contract

The Factory manages markets and global parameters.

1.  Go to [cspr.live/deploy](https://testnet.cspr.live/deploy).
2.  Upload `wasm/MarketFactory.wasm`.
3.  **Arguments**:
    - `admin` (Type: `Key`): Enter your account address.
    - `vault_contract` (Type: `Key`): Enter the **Vault Contract Hash** from Step 1.
    - `platform_fee_bps` (Type: `U64`): `200` (for 2% fee).
    - `min_market_duration` (Type: `U64`): `3600` (1 hour).
    - `max_market_duration` (Type: `U64`): `2592000` (30 days).
    - `min_initial_liquidity` (Type: `U256`): `1000000000` (1 CSPR).
4.  Sign and submit.
5.  Wait for execution. **Copy the Factory Contract Hash**.

### 3. Authorize Factory in Vault

You must tell the Vault to accept calls from the Factory.

1.  Go to the **Vault Contract** page on cspr.live (search by the hash from Step 1).
2.  Go to the **Entry Points** or **Write Contract** tab.
3.  Select `set_factory`.
4.  **Arguments**:
    - `factory` (Type: `Key`): Enter the **Factory Contract Hash** from Step 2.
5.  Sign and submit.

### 4. (Optional) Deploy a Standalone Market

To test a single market directly without the factory creating it (deployment automation):

1.  Go to [cspr.live/deploy](https://testnet.cspr.live/deploy).
2.  Upload `wasm/Market.wasm`.
3.  **Arguments**:
    - Fill in `MarketInitArgs` (this is complex via UI, simplified below):
        - `market_id`: `1`
        - `market_type`: `0` (Binary)
        - `question`: "Will it rain?"
        - `outcome_names`: List of strings (may be hard in UI, try keeping it simple).
        - `end_time`: Unix timestamp (e.g., current time + 86400).
        - `creator`: Your Key.
        - `admin`: Your Key.
        - `resolver`: Your Key.
        - `vault_contract`: Vault Hash.
        - `factory_contract`: Factory Hash.
        - `platform_fee_bps`: `200`.
        - `bonding_params`: (Complex struct, may need default).

> **Note**: Deploying complex structs like `BondingCurveParams` via the web UI can be difficult. It is recommended to rely on the Factory or use the JS SDK/Casper Client for market creation.

## Verification

Once deployed, you can verify the setup:

1.  Check `get_factory` on the Vault contract; it should return your Factory hash.
2.  Check `get_admin` on both contracts.

## Troubleshooting

- **"Out of Gas"**: Increase the payment amount (Standard payment is usually sufficient, but large contracts may need 150-200 CSPR).
- **"Invalid Argument"**: Double-check types (Key vs Address vs String).
