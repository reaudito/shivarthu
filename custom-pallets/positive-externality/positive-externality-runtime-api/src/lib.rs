#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::sp_std::{vec::Vec};
//  or
use parity_scale_codec::Codec;
use sp_std::prelude::*;

sp_api::decl_runtime_apis! {
	pub trait PositiveExternalityApi<AccountId> where AccountId: Codec {

		fn get_evidence_period_end_block(user_to_calculate: AccountId) -> Option<u32>;
		fn get_staking_period_end_block(user_to_calculate: AccountId) -> Option<u32>;
		fn get_drawing_period_end(user_to_calculate: AccountId) -> (u64, u64, bool);
		fn get_commit_period_end_block(user_to_calculate: AccountId) -> Option<u32>;
		fn get_vote_period_end_block(user_to_calculate: AccountId) -> Option<u32>;
		fn selected_as_juror(user_to_calculate: AccountId, who: AccountId) -> bool;
		fn post_by_address_length(user: AccountId) -> u64;
		fn paginate_posts_by_address(user: AccountId, page: u64, page_size: u64) -> Option<Vec<u64>>;
		fn paginate_posts_by_address_latest(user: AccountId, page: u64, page_size: u64) -> Option<Vec<u64>>;
		fn validation_list_length() -> u64;
		fn validation_list_latest(page: u64, page_size: u64) -> Option<Vec<AccountId>>;
		fn has_user_staked(user_to_calculate: AccountId, who: AccountId) -> bool;
		fn user_staked_value(user_to_calculate: AccountId, who: AccountId) -> u64;
	}
}
