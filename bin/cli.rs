//! CLI tool for deploying and interacting with Casper Predict contracts.

use casper_predict::{
    factory::{FactoryInitArgs, MarketFactory},
    vault::{Vault, VaultInitArgs},
};
use odra::casper_types::U256;
use odra::host::{HostEnv, HostRef};
use odra_cli::{
    deploy::DeployScript,
    scenario::{Args, Error, Scenario, ScenarioMetadata},
    CommandArg, DeployedContractsContainer, OdraCli,
};

/// Deploys the Vault and MarketFactory contracts and sets up permissions.
pub struct DeploySystemScript;

impl DeployScript for DeploySystemScript {
    fn deploy(
        &self,
        env: &HostEnv,
        container: &mut DeployedContractsContainer,
    ) -> Result<(), odra_cli::deploy::Error> {
        let admin = env.get_account(0);
        let fee_recipient = admin; // Default to admin receiving fees

        println!("Deploying Vault...");
        let vault_args = VaultInitArgs {
            admin,
            fee_recipient,
        };
        // Gas limit: 150 CSPR (150 * 10^9)
        let mut vault = Vault::load_or_deploy(env, vault_args, container, 150_000_000_000)?;
        println!("Vault deployed at: {:?}", vault.address());

        println!("Deploying MarketFactory...");
        let factory_args = FactoryInitArgs {
            admin,
            vault_contract: vault.address(),
            platform_fee_bps: 200,                // 2%
            min_market_duration: 3600,            // 1 hour
            max_market_duration: 2592000,         // 30 days
            min_initial_liquidity: U256::from(1_000_000_000u64), // 1 CSPR
        };
        // Gas limit: 200 CSPR
        let factory = MarketFactory::load_or_deploy(env, factory_args, container, 200_000_000_000)?;
        println!("MarketFactory deployed at: {:?}", factory.address());

        println!("Title: Authorizing Factory in Vault...");
        // Set gas for the transaction
        env.set_gas(5_000_000_000); // 5 CSPR
        vault.authorize_market(*factory.address());
        println!("Factory authorized.");

        println!("System deployment complete!");
        Ok(())
    }
}

pub fn main() {
    OdraCli::new()
        .about("Casper Predict Deployment CLI")
        .deploy(DeploySystemScript)
        .contract::<Vault>()
        .contract::<MarketFactory>()
        .build()
        .run();
}
