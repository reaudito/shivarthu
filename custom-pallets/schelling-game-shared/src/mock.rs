use crate as pallet_template;
use frame_support::derive_impl;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::BuildStorage;
use sp_std::vec;

type Block = frame_system::mocking::MockBlock<Test>;

pub struct TestRandomness<T>(core::marker::PhantomData<T>);

impl<Output: codec::Decode + Default, T>
    frame_support::traits::Randomness<Output, BlockNumberFor<T>> for TestRandomness<T>
where
    T: frame_system::Config,
{
    fn random(subject: &[u8]) -> (Output, BlockNumberFor<T>) {
        use sp_runtime::traits::TrailingZeroInput;

        (
            Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
            frame_system::Pallet::<T>::block_number(),
        )
    }
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
    pub type TemplateModule = pallet_template::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type Balances = pallet_balances::Pallet<Test>;

    #[runtime::pallet_index(3)]
    pub type SortitionSumGame = pallet_sortition_sum_game::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<u64>;
}

impl pallet_template::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances; // New code
    type RandomnessSource = TestRandomness<Self>;
    type Slash = ();
    type Reward = ();
    type SortitionSumGameSource = SortitionSumGame;
}

impl pallet_sortition_sum_game::Config for Test {
    type RuntimeEvent = RuntimeEvent;
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
    t.into()
}
