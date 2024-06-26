#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::sp_std::{vec::Vec};
//  or
use parity_scale_codec::Codec;
use sp_std::prelude::*;

type ProjectId = u64;

sp_api::decl_runtime_apis! {
    pub trait ProjectTipsApi<AccountId> where AccountId: Codec{

        fn get_evidence_period_end_block(project_id: ProjectId) -> Option<u32>;
        fn get_staking_period_end_block(project_id: ProjectId) -> Option<u32>;
        fn get_drawing_period_end(project_id: ProjectId) -> (u64, u64, bool);
        fn get_commit_period_end_block(project_id: ProjectId) -> Option<u32>;
        fn get_vote_period_end_block(project_id: ProjectId) -> Option<u32>;
        fn selected_as_juror(project_id: ProjectId, who: AccountId) -> bool;
    }
}
