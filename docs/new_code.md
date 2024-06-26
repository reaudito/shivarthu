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
