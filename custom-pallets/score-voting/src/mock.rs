use crate as pallet_template;
use frame_support::pallet_prelude::*;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU16, ConstU64},
};
use frame_system::pallet_prelude::*;
use std::collections::HashMap;
use trait_shared_storage::SharedStorageLink;

use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

use std::cell::RefCell;
thread_local! {
    static MOCK_MEMBERSHIP: RefCell<HashMap<(u64, u64), bool>> = RefCell::new(HashMap::new());
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
    pub type Timestamp = pallet_timestamp::Pallet<Test>;

    #[runtime::pallet_index(4)]
    pub type SharedStorage = pallet_shared_storage::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<u64>;
}

pub struct MockSharedStorage;

impl SharedStorageLink for MockSharedStorage {
    type AccountId = u64;

    fn add_approved_citizen_address(_new_member: Self::AccountId) -> DispatchResult {
        unimplemented!("add_approved_citizen_address is not mocked")
    }

    fn check_citizen_is_approved_link(_address: Self::AccountId) -> DispatchResult {
        unimplemented!("check_citizen_is_approved_link is not mocked")
    }

    fn get_approved_citizen_count_link() -> u64 {
        unimplemented!("get_approved_citizen_count_link is not mocked")
    }

    fn set_positive_externality_link(_address: Self::AccountId, _score: i64) -> DispatchResult {
        unimplemented!("set_positive_externality_link is not mocked")
    }

    fn add_reputation_score_to_department(
        _address: Self::AccountId,
        _department_id: u64,
        _amount: i64,
    ) -> DispatchResult {
        unimplemented!("add_reputation_score_to_department is not mocked")
    }

    fn get_department_reputation_score(
        _address: Self::AccountId,
        _department_id: u64,
    ) -> Option<i64> {
        unimplemented!("get_department_reputation_score is not mocked")
    }

    fn get_total_reputation_score(_address: Self::AccountId) -> i64 {
        unimplemented!("get_total_reputation_score is not mocked")
    }

    fn is_member_in_group_district(
        _group_id: u64,
        _member: Self::AccountId,
    ) -> Result<bool, DispatchError> {
        unimplemented!("is_member_in_group_district is not mocked")
    }

    fn is_member_in_group_specialization(
        _group_id: u64,
        _member: Self::AccountId,
    ) -> Result<bool, DispatchError> {
        unimplemented!("is_member_in_group_specialization is not mocked")
    }

    fn is_member_and_score_in_group_specialization(
        _group_id: u64,
        _member: Self::AccountId,
    ) -> Result<(bool, i64), DispatchError> {
        unimplemented!("is_member_and_score_in_group_specialization is not mocked")
    }

    fn are_district_departments_empty(_group_id: u64) -> Result<bool, DispatchError> {
        unimplemented!("are_district_departments_empty is not mocked")
    }

    fn are_specialization_departments_empty(_group_id: u64) -> Result<bool, DispatchError> {
        unimplemented!("are_specialization_departments_empty is not mocked")
    }

    fn is_member_in_group_district_and_specialization(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<bool, DispatchError> {
        Ok(MOCK_MEMBERSHIP.with(|m| *m.borrow().get(&(group_id, member)).unwrap_or(&false)))
    }
}

pub fn set_mock_membership(group_id: u64, member: u64, value: bool) {
    MOCK_MEMBERSHIP.with(|m| {
        m.borrow_mut().insert((group_id, member), value);
    });
}

pub fn initialize_mock_members() {
    set_mock_membership(1, 1, true);
    set_mock_membership(1, 2, true);
    set_mock_membership(1, 3, true);
    set_mock_membership(1, 4, true);
    set_mock_membership(1, 5, true);
    set_mock_membership(1, 6, true);
    set_mock_membership(1, 7, true);
    set_mock_membership(1, 8, true);

    set_mock_membership(2, 1, true);
    set_mock_membership(2, 2, true);
    set_mock_membership(2, 3, true);
    set_mock_membership(2, 4, true);
    set_mock_membership(2, 5, true);
    set_mock_membership(2, 6, true);
    set_mock_membership(2, 7, true);
    set_mock_membership(2, 8, true);
}

impl pallet_template::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type SharedStorageSource = MockSharedStorage;
    type Currency = Balances; // New code
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

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type AccountStore = System;
}

impl pallet_shared_storage::Config for Test {
    type RuntimeEvent = RuntimeEvent;
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
