# New codes


## Main Cargo.toml

```toml

[workspace]
members = [
	"custom-pallets/*",
]

```

```toml
[dependencies]
## New start

## New pallets
pallet-template = {path = "custom-pallets/template", default-features= false}
pallet-support = {path = "custom-pallets/support", default-features= false}
pallet-spaces = {path= "custom-pallets/spaces", default-features=false }
pallet-sortition-sum-game = {path = "custom-pallets/sortition-sum-game", default-features=false}
pallet-shared-storage = {path = "custom-pallets/shared-storage", default-features = false}
pallet-schelling-game-shared = {path = "custom-pallets/schelling-game-shared", default-features = false }
pallet-profile-validation = {path = "custom-pallets/profile-validation", default-features = false}
pallet-project-tips = {path = "custom-pallets/project-tips", default-features = false}
pallet-positive-externality = { path = "custom-pallets/positive-externality", default-features = false }
pallet-department-funding = { path = "custom-pallets/department-funding", default-features = false }

## Traits
trait-sortition-sum-game = {path = "traits/trait-sortition-sum-game", default-features=false}
trait-shared-storage = {path= "traits/trait-shared-storage", default-features=false}
trait-schelling-game-shared = {path = "traits/trait-schelling-game-shared", default-features=false}

## Api
profile-validation-runtime-api = { path = "custom-pallets/profile-validation/profile-validation-runtime-api", default-features = false}
project-tips-runtime-api = { path = "custom-pallets/project-tips/project-tips-runtime-api", default-features = false }
positive-externality-runtime-api = { path = "custom-pallets/positive-externality/positive-externality-runtime-api", default-features = false}
department-funding-runtime-api = { path = "custom-pallets/department-funding/department-funding-runtime-api", default-features = false}


## Rpc
profile-validation-rpc = { path = "custom-pallets/profile-validation/profile-validation-rpc", default-features = false}
project-tips-rpc = { path = "custom-pallets/project-tips/project-tips-rpc", default-features = false }
positive-externality-rpc = { path = "custom-pallets/positive-externality/positive-externality-rpc", default-features = false}
department-funding-rpc = { path = "custom-pallets/department-funding/department-funding-rpc", default-features = false}




## Additional dependancies
sp-arithmetic = { git = "https://github.com/moondance-labs/polkadot-sdk", branch = "tanssi-polkadot-v1.6.0", default-features = false  }
strum = { version = "0.26.2", default-features = false, features = ["derive"] }
num-integer = {default-features = false, version= "0.1.44"}
frame-support-test ={ git = "https://github.com/moondance-labs/polkadot-sdk", branch = "tanssi-polkadot-v1.6.0", default-features = false}
pallet-insecure-randomness-collective-flip = { git = "https://github.com/moondance-labs/polkadot-sdk", branch = "tanssi-polkadot-v1.6.0", default-features = false}
sp-npos-elections = { git = "https://github.com/moondance-labs/polkadot-sdk", branch = "tanssi-polkadot-v1.6.0", default-features = false }
## New end


```


## Node runtime

### **node-runtime/Cargo.toml**

```toml
[dependencies]
pallet-insecure-randomness-collective-flip = { workspace = true  }


# Local Dependencies
pallet-template = {  workspace = true }

pallet-sortition-sum-game = { workspace = true }
# pallet-election={  workspace = true }
# # election-runtime-api={ workspace = true }
# pallet-posts = { workspace = true }
# pallet-spaces = { workspace = true }
pallet-schelling-game-shared = { workspace = true }
pallet-profile-validation = { workspace = true }
# profile-validation-runtime-api = {  workspace = true }
pallet-shared-storage = { workspace = true }
pallet-positive-externality = { workspace = true }
pallet-department-funding = {  workspace = true }
pallet-project-tips = { workspace = true }

profile-validation-runtime-api = { workspace = true }
positive-externality-runtime-api = {  workspace = true }
department-funding-runtime-api = { workspace = true }
project-tips-runtime-api = {  workspace = true }


[features]
default = [ "std" ]
std = [
   "pallet-insecure-randomness-collective-flip/std",
   "pallet-template/std",
   "pallet-sortition-sum-game/std",
   "pallet-schelling-game-shared/std",
   "pallet-profile-validation/std",
   "pallet-shared-storage/std",
   "pallet-positive-externality/std",
   "pallet-department-funding/std",
   "pallet-project-tips/std",
   "profile-validation-runtime-api/std",
   "positive-externality-runtime-api/std",
   "department-funding-runtime-api/std",
   "project-tips-runtime-api/std",
]

runtime-benchmarks = [
   "pallet-template/runtime-benchmarks",
   "pallet-sortition-sum-game/runtime-benchmarks",
   "pallet-schelling-game-shared/runtime-benchmarks",
   "pallet-profile-validation/runtime-benchmarks",
   "pallet-shared-storage/runtime-benchmarks",
   "pallet-positive-externality/runtime-benchmarks",
   "pallet-department-funding/runtime-benchmarks",
   "pallet-project-tips/runtime-benchmarks",
]

try-runtime = [
   "pallet-template/try-runtime",
   "pallet-sortition-sum-game/try-runtime",
   "pallet-schelling-game-shared/try-runtime",
   "pallet-profile-validation/try-runtime",
   "pallet-shared-storage/try-runtime",
   "pallet-positive-externality/try-runtime",
   "pallet-department-funding/try-runtime",
   "pallet-project-tips/try-runtime",
]

```

### **runtime-templates/simple/src/lib.rs**

```rust

impl pallet_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_template::weights::SubstrateWeight<Runtime>;
}

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}


impl pallet_sortition_sum_game::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_sortition_sum_game::weights::SubstrateWeight<Runtime>;
}

impl pallet_schelling_game_shared::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_schelling_game_shared::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type RandomnessSource = RandomnessCollectiveFlip;
	type Slash = ();
	type Reward = ();
	type SortitionSumGameSource = SortitionSumGame;
}

impl pallet_profile_validation::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_profile_validation::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type SchellingGameSharedSource = SchellingGameShared;
	type Slash = ();
	type Reward = ();
}


impl pallet_shared_storage::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_shared_storage::weights::SubstrateWeight<Runtime>;
}

impl pallet_positive_externality::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_positive_externality::weights::SubstrateWeight<Runtime>;
	type SharedStorageSource = SharedStorage;
	type Currency = Balances;
	type SchellingGameSharedSource = SchellingGameShared;
	type Reward = ();
}

impl pallet_department_funding::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_department_funding::weights::SubstrateWeight<Runtime>;
	type SharedStorageSource = SharedStorage;
	type Currency = Balances;
	type SchellingGameSharedSource = SchellingGameShared;
}

impl pallet_project_tips::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_project_tips::weights::SubstrateWeight<Runtime>;
	type SharedStorageSource = SharedStorage;
	type Currency = Balances;
	type Reward = ();
	type SchellingGameSharedSource = SchellingGameShared;	
}


construct_runtime!(
    pub enum Runtime
    {
        TemplateModule: pallet_template = 200,
		SortitionSumGame: pallet_sortition_sum_game = 201,
		SchellingGameShared: pallet_schelling_game_shared = 202,
		ProfileValidation: pallet_profile_validation = 203,
		SharedStorage: pallet_shared_storage = 204,
		PositiveExternality: pallet_positive_externality = 205,
		DepartmentFunding: pallet_department_funding = 206,
		ProjectTips: pallet_project_tips = 207,
        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip = 208,
    }
);

```

## Runtime apis
*runtime-templates/simple/src/lib.rs*

```rust

pub type ChallengePostId = u64;

pub type DepartmentRequiredFundId = u64;

pub type ProjectId = u64;


impl_runtime_apis! {
    impl profile_validation_runtime_api::ProfileValidationApi<Block, AccountId> for Runtime {

		fn get_challengers_evidence(profile_user_account: AccountId, offset: u64, limit: u16) -> Vec<ChallengePostId> {
			ProfileValidation::get_challengers_evidence(profile_user_account, offset, limit)
		}

		fn get_evidence_period_end_block(profile_user_account: AccountId) -> Option<u32> {
			ProfileValidation::get_evidence_period_end_block(profile_user_account)
		}

		fn get_staking_period_end_block(profile_user_account: AccountId) -> Option<u32> {
			ProfileValidation::get_staking_period_end_block(profile_user_account)
		}
		fn get_drawing_period_end(profile_user_account: AccountId) -> (u64, u64, bool) {
			ProfileValidation::get_drawing_period_end(profile_user_account)
		}
		fn get_commit_period_end_block(profile_user_account: AccountId) -> Option<u32> {
			ProfileValidation::get_commit_period_end_block(profile_user_account)
		}

		fn get_vote_period_end_block(profile_user_account: AccountId) -> Option<u32> {
			ProfileValidation::get_vote_period_end_block(profile_user_account)
		}
		fn selected_as_juror(profile_user_account: AccountId, who: AccountId) -> bool {
			ProfileValidation::selected_as_juror(profile_user_account, who)
		}
	}

	impl department_funding_runtime_api::DepartmentFundingApi<Block, AccountId> for Runtime {

		fn get_evidence_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32> {
			DepartmentFunding::get_evidence_period_end_block(department_required_fund_id)
		}

		fn get_staking_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32> {
			DepartmentFunding::get_staking_period_end_block(department_required_fund_id)
		}
		fn get_drawing_period_end(department_required_fund_id: DepartmentRequiredFundId) -> (u64, u64, bool) {
			DepartmentFunding::get_drawing_period_end(department_required_fund_id)
		}
		fn get_commit_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32> {
			DepartmentFunding::get_commit_period_end_block(department_required_fund_id)
		}

		fn get_vote_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32> {
			DepartmentFunding::get_vote_period_end_block(department_required_fund_id)
		}
		fn selected_as_juror(department_required_fund_id: DepartmentRequiredFundId, who: AccountId) -> bool {
			DepartmentFunding::selected_as_juror(department_required_fund_id, who)
		}
	}

	impl positive_externality_runtime_api::PositiveExternalityApi<Block, AccountId> for Runtime {

		fn get_evidence_period_end_block(user_to_calculate: AccountId) -> Option<u32> {
			PositiveExternality::get_evidence_period_end_block(user_to_calculate)
		}

		fn get_staking_period_end_block(user_to_calculate: AccountId) -> Option<u32> {
			PositiveExternality::get_staking_period_end_block(user_to_calculate)
		}
		fn get_drawing_period_end(user_to_calculate: AccountId) -> (u64, u64, bool) {
			PositiveExternality::get_drawing_period_end(user_to_calculate)
		}
		fn get_commit_period_end_block(user_to_calculate: AccountId) -> Option<u32> {
			PositiveExternality::get_commit_period_end_block(user_to_calculate)
		}

		fn get_vote_period_end_block(user_to_calculate: AccountId) -> Option<u32> {
			PositiveExternality::get_vote_period_end_block(user_to_calculate)
		}
		fn selected_as_juror(user_to_calculate: AccountId, who: AccountId) -> bool {
			PositiveExternality::selected_as_juror(user_to_calculate, who)
		}
	}

	impl project_tips_runtime_api::ProjectTipsApi<Block, AccountId> for Runtime {

		fn get_evidence_period_end_block(project_id: ProjectId) -> Option<u32> {
			ProjectTips::get_evidence_period_end_block(project_id)
		}

		fn get_staking_period_end_block(project_id: ProjectId) -> Option<u32> {
			ProjectTips::get_staking_period_end_block(project_id)
		}
		fn get_drawing_period_end(project_id: ProjectId) -> (u64, u64, bool) {
			ProjectTips::get_drawing_period_end(project_id)
		}
		fn get_commit_period_end_block(project_id: ProjectId) -> Option<u32> {
			ProjectTips::get_commit_period_end_block(project_id)
		}

		fn get_vote_period_end_block(project_id: ProjectId) -> Option<u32> {
			ProjectTips::get_vote_period_end_block(project_id)
		}
		fn selected_as_juror(project_id: ProjectId, who: AccountId) -> bool {
			ProjectTips::selected_as_juror(project_id, who)
		}
	}
```

### **nodes/simple/src/chain_spec.rs** 

```rust
fn testnet_genesis(
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
) -> serde_json::Value {
    let g = container_chain_template_simple_runtime::RuntimeGenesisConfig {
         shared_storage: Default::default(),
    };
```

### **nodes/simple/src/Cargo.toml**

```toml
pallet-shared-storage = { workspace = true }
```

## node rpc

*container-chains/nodes/simple/rpc.rs*


```rust
/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: profile_validation_runtime_api::ProfileValidationApi<Block, AccountId>,
	C::Api: department_funding_runtime_api::DepartmentFundingApi<Block, AccountId>,
	C::Api: positive_externality_runtime_api::PositiveExternalityApi<Block, AccountId>,
	C::Api: project_tips_runtime_api::ProjectTipsApi<Block, AccountId>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
{
	use department_funding_rpc::DepartmentFundingApiServer;
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use positive_externality_rpc::PositiveExternalityApiServer;
	use profile_validation_rpc::ProfileValidationApiServer;
	use project_tips_rpc::ProjectTipsApiServer;
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(profile_validation_rpc::ProfileValidation::new(client.clone()).into_rpc())?;
	module.merge(department_funding_rpc::DepartmentFunding::new(client.clone()).into_rpc())?;
	module.merge(positive_externality_rpc::PositiveExternality::new(client.clone()).into_rpc())?;
	module.merge(project_tips_rpc::ProjectTips::new(client.clone()).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	Ok(module)
}

```

## Node rpc new code

```rust
/// Instantiate all RPC extensions.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + Sync + Send + 'static,
    C::Api: profile_validation_runtime_api::ProfileValidationApi<Block, AccountId>,
	C::Api: department_funding_runtime_api::DepartmentFundingApi<Block, AccountId>,
	C::Api: positive_externality_runtime_api::PositiveExternalityApi<Block, AccountId>,
	C::Api: project_tips_runtime_api::ProjectTipsApi<Block, AccountId>,
{
    use substrate_frame_rpc_system::{System, SystemApiServer};
    use department_funding_rpc::DepartmentFundingApiServer;
	use positive_externality_rpc::PositiveExternalityApiServer;
	use profile_validation_rpc::ProfileValidationApiServer;
	use project_tips_rpc::ProjectTipsApiServer;

    let mut module = RpcExtension::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
        command_sink,
        xcm_senders,
    } = deps;

    module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;

    // Manual seal
    if let Some(command_sink) = command_sink {
        module.merge(
            // We provide the rpc handler with the sending end of the channel to allow the rpc
            // send EngineCommands to the background block authorship task.
            ManualSeal::new(command_sink).into_rpc(),
        )?;
    };

    if let Some((downward_message_channel, hrmp_message_channel)) = xcm_senders {
        module.merge(
            ManualXcm {
                downward_message_channel,
                hrmp_message_channel,
            }
            .into_rpc(),
        )?;
    }

    module.merge(profile_validation_rpc::ProfileValidation::new(client.clone()).into_rpc())?;
	module.merge(department_funding_rpc::DepartmentFunding::new(client.clone()).into_rpc())?;
	module.merge(positive_externality_rpc::PositiveExternality::new(client.clone()).into_rpc())?;
	module.merge(project_tips_rpc::ProjectTips::new(client.clone()).into_rpc())?;

    Ok(module)
}
```

*container-chains/nodes/simple/Cargo.toml*

```toml
# profile valdiation rpc
profile-validation-runtime-api = { workspace = true }
profile-validation-rpc = { workspace = true }


# Department funding rpc
department-funding-runtime-api = { workspace = true }
department-funding-rpc= {workspace = true }

# Postive exterality rpc
positive-externality-runtime-api = { workspace = true }
positive-externality-rpc= { workspace = true }

# Project tip rpc
project-tips-runtime-api = { workspace = true }
project-tips-rpc= { workspace = true }
```

## Chain spec

*container-chains/nodes/simple/chain_spec.rs*


```rust
/// Get pre-funded accounts
pub fn pre_funded_accounts() -> Vec<AccountId> {
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
    ]
}
```


