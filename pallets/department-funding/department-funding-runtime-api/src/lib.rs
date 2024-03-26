#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::sp_std::{vec::Vec};
//  or
use frame_support::sp_std::{prelude::*};
use sp_api::codec::Codec;
type ChallengePostId = u64;
type DepartmentRequiredFundId= u64;

sp_api::decl_runtime_apis! {
	pub trait DepartmentFundingApi<AccountId> where AccountId: Codec {
		fn get_challengers_evidence(department_required_fund_id: DepartmentRequiredFundId, offset: u64, limit: u16) -> Vec<ChallengePostId>;
		fn get_evidence_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32>;
		fn get_staking_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32>;
		fn get_drawing_period_end(department_required_fund_id: DepartmentRequiredFundId) -> (u64, u64, bool);
		fn get_commit_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32>;
		fn get_vote_period_end_block(department_required_fund_id: DepartmentRequiredFundId) -> Option<u32>;
		fn selected_as_juror(department_required_fund_id: DepartmentRequiredFundId, who: AccountId) -> bool;
	}
}