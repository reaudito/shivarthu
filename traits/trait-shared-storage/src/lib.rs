#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};

pub trait SharedStorageLink {
    type AccountId;

    fn add_approved_citizen_address(new_member: Self::AccountId) -> DispatchResult;

    fn check_citizen_is_approved_link(address: Self::AccountId) -> DispatchResult;

    fn get_approved_citizen_count_link() -> u64;
    fn set_positive_externality_link(address: Self::AccountId, score: i64) -> DispatchResult;
    fn add_reputation_score_to_department(
        address: Self::AccountId,
        department_id: u64,
        amount: i64,
    ) -> DispatchResult;

    fn get_department_reputation_score(address: Self::AccountId, department_id: u64)
        -> Option<i64>;

    fn get_total_reputation_score(address: Self::AccountId) -> i64;

    fn is_member_in_group_district(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<bool, DispatchError>;

    fn is_member_in_group_specialization(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<bool, DispatchError>;

    fn is_member_and_score_in_group_specialization(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<(bool, i64), DispatchError>;
    fn are_district_departments_empty(group_id: u64) -> Result<bool, DispatchError>;

    fn are_specialization_departments_empty(group_id: u64) -> Result<bool, DispatchError>;

    fn is_member_in_group_district_and_specialization(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<bool, DispatchError>;
}
