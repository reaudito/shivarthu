use crate as pallet_template;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU16, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
use frame_support_test::TestRandomness;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        ProjectTips: pallet_template,
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
    type AccountData = pallet_balances::AccountData<u64>; // New code
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

impl pallet_shared_storage::Config for Test {
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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 1000000),
            (2, 2000000),
            (3, 3000000),
            (4, 3000000),
            (5, 3000000),
            (6, 3000000),
            (7, 3000000),
            (8, 3000000),
            (9, 3000000),
            (10, 3000000),
            (11, 3000000),
            (12, 3000000),
            (13, 3000000),
            (14, 3000000),
            (15, 3000000),
            (16, 3000000),
            (17, 3000000),
            (18, 3000000),
            (19, 3000000),
            (20, 3000000),
            (21, 3000000),
            (22, 3000000),
            (23, 3000000),
            (24, 3000000),
            (25, 3000000),
            (26, 3000000),
            (27, 3000000),
            (28, 3000000),
            (29, 3000000),
            (30, 3000000),
            (31, 3000000),
            (32, 3000000),
            (33, 3000000),
            (34, 3000000),
            (35, 3000000),
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
