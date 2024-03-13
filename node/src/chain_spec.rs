use hex_literal::hex;
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, SharedStorageConfig,
	Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					hex!("2e778beae3cc11fd7ea694f4ff8b54922d67e0599c356f393277ed9711d6364b").into(),
					hex!("2e1c14cd13a2b090a62203809d8ce3eaac7417a4a0272438568eb04cae330669").into(),
					hex!("ba0ce278d82ef9a686cb60a801125a8d11b32caa2456ebdcfe7ff687bb9bf540").into(),
					hex!("600f10bdbf233ac6614eea62ae45d269b43c759e4ddf0bc1a70ffcbc95499c6c").into(),
					hex!("c2da35a7aed402249295971abe8f10e0b03d861a0571e56115bcc6f8828dd939").into(),
					hex!("186863b612097dec4ce7b9772381935baa7fc6dc7c44695f0384174f1b131156").into(),
					hex!("70c3f87a26743fed9194f8fc67bfdd9a211f3b00f5c80459107022d096dbf928").into(),
					hex!("cab4abef5dda97cc98eb0f3a5e0329bd2c1b892b5f442021a634c7e79e6f6e29").into(),
					hex!("ac926b4e81989ca51c9ac6f0ef9c7db08d5334bb0a5c3b0194bf92d215b50f3f").into(),
					hex!("186c72f04de9c1a74cee6836c08b6d56a88e90ab5a6127693a55379e8e03d919").into(),
					hex!("b02de28c52fe59f9a3d8779cd8c6ee7439cba45e48e7ee582f5cc939c7b5946c").into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					hex!("2e778beae3cc11fd7ea694f4ff8b54922d67e0599c356f393277ed9711d6364b").into(),
					hex!("2e1c14cd13a2b090a62203809d8ce3eaac7417a4a0272438568eb04cae330669").into(),
					hex!("ba0ce278d82ef9a686cb60a801125a8d11b32caa2456ebdcfe7ff687bb9bf540").into(),
					hex!("600f10bdbf233ac6614eea62ae45d269b43c759e4ddf0bc1a70ffcbc95499c6c").into(),
					hex!("c2da35a7aed402249295971abe8f10e0b03d861a0571e56115bcc6f8828dd939").into(),
					hex!("186863b612097dec4ce7b9772381935baa7fc6dc7c44695f0384174f1b131156").into(),
					hex!("70c3f87a26743fed9194f8fc67bfdd9a211f3b00f5c80459107022d096dbf928").into(),
					hex!("cab4abef5dda97cc98eb0f3a5e0329bd2c1b892b5f442021a634c7e79e6f6e29").into(),
					hex!("ac926b4e81989ca51c9ac6f0ef9c7db08d5334bb0a5c3b0194bf92d215b50f3f").into(),
					hex!("186c72f04de9c1a74cee6836c08b6d56a88e90ab5a6127693a55379e8e03d919").into(),
					hex!("b02de28c52fe59f9a3d8779cd8c6ee7439cba45e48e7ee582f5cc939c7b5946c").into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		shared_storage: SharedStorageConfig { approved_citizen_address: endowed_accounts },
	}
}
