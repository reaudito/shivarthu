use crate as pallet_template;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU16, ConstU64},
};
use frame_support_test::TestRandomness;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        TemplateModule: pallet_template,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        SharedStorage:pallet_shared_storage,
        SchellingGameShared: pallet_schelling_game_shared,
        SortitionSumGame: pallet_sortition_sum_game,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type AccountData = pallet_balances::AccountData<u64>; // N
}

impl pallet_shared_storage::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}
parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_balances::Config for Test {
    type MaxHolds = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

impl pallet_schelling_game_shared::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Currency = Balances; // New code
    type RandomnessSource = TestRandomness<Self>;
    type Slash = ();
    type Reward = ();
    type SortitionSumGameSource = SortitionSumGame;
}

impl pallet_sortition_sum_game::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_template::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type SharedStorageSource = SharedStorage;
    type Currency = Balances; // New code
    type SchellingGameSharedSource = SchellingGameShared;
    type Reward = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 100000),
            (2, 200000),
            (3, 300000),
            (4, 300000),
            (5, 300000),
            (6, 300000),
            (7, 300000),
            (8, 300000),
            (9, 300000),
            (10, 300000),
            (11, 300000),
            (12, 300000),
            (13, 300000),
            (14, 300000),
            (15, 300000),
            (16, 300000),
            (17, 300000),
            (18, 300000),
            (19, 300000),
            (20, 300000),
            (21, 300000),
            (22, 300000),
            (23, 300000),
            (24, 300000),
            (25, 300000),
            (26, 300000),
            (27, 300000),
            (28, 300000),
            (29, 300000),
            (30, 300000),
            (31, 300000),
            (32, 300000),
            (33, 300000),
            (34, 300000),
            (35, 300000),
        ],
    } // new code
    .assimilate_storage(&mut t)
    .unwrap();
    pallet_shared_storage::GenesisConfig::<Test> {
        approved_citizen_address: vec![1, 2],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}
