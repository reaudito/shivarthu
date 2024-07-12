#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_prelude::DispatchResult;
use sp_std::vec::Vec;

pub trait SharedStorageLink {
    type AccountId;

    fn check_citizen_is_approved_link(address: Self::AccountId) -> DispatchResult;

    fn get_approved_citizen_count_link() -> u64;
    fn set_positive_externality_link(address: Self::AccountId, score: i64) -> DispatchResult;
    fn add_reputation_score_to_department(
        address: Self::AccountId,
        department: Vec<u8>,
        amount: i64,
    ) -> DispatchResult;
    fn subtract_reputation_score_from_department(
        address: Self::AccountId,
        department: Vec<u8>,
        amount: i64,
    ) -> DispatchResult;

    fn get_department_reputation_score(
        address: Self::AccountId,
        department: Vec<u8>,
    ) -> Option<i64>;

    fn get_total_reputation_score(address: Self::AccountId) -> i64;
}
