use crate as pallet_template;
use frame_support::traits::LockIdentifier;
use frame_support::{derive_impl, parameter_types};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

pub const UNITS: u64 = 1_000_000;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaxLocks: u32 = 50;
    pub const MaxLockId: LockIdentifier = *b"cvote_id";
}
#[frame_support::runtime]
mod runtime {
    // The main runtime
    #[runtime::runtime]
    // Runtime Types to be generated
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Template = pallet_template::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type Balances = pallet_balances::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<u64>;
}

impl pallet_template::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxLockId = MaxLockId;
    type WeightInfo = ();
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type AccountStore = System;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 1000 * UNITS),
            (2, 2000 * UNITS),
            (3, 3000 * UNITS),
            (4, 3000 * UNITS),
            (5, 3000 * UNITS),
            (6, 3000 * UNITS),
            (7, 3000 * UNITS),
            (8, 3000 * UNITS),
            (9, 3000 * UNITS),
            (10, 3000 * UNITS),
            (11, 3000 * UNITS),
            (12, 3000 * UNITS),
            (13, 3000 * UNITS),
            (14, 3000 * UNITS),
            (15, 3000 * UNITS),
            (16, 3000 * UNITS),
            (17, 3000 * UNITS),
            (18, 3000 * UNITS),
            (19, 3000 * UNITS),
            (20, 3000 * UNITS),
            (21, 3000 * UNITS),
            (22, 3000 * UNITS),
            (23, 3000 * UNITS),
            (24, 3000 * UNITS),
            (25, 3000 * UNITS),
            (26, 3000 * UNITS),
            (27, 3000 * UNITS),
            (28, 3000 * UNITS),
            (29, 3000 * UNITS),
            (30, 3000 * UNITS),
            (31, 3000 * UNITS),
            (32, 3000 * UNITS),
            (33, 3000 * UNITS),
            (34, 3000 * UNITS),
            (35, 3000 * UNITS),
        ],
    } // new code
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}
