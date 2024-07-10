use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use frame_support::pallet_prelude::*;

use super::*;

pub const FIRST_POST_ID: u64 = 1;

/// Information about a post's owner, its' related space, content, and visibility.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Post<T: Config> {
    pub id: PostId,

    pub created: WhoAndWhenOf<T>,

    pub edited: bool,

    pub owner: T::AccountId,

    pub content: Content,

    pub hidden: bool,

    pub upvotes_count: u32,

    pub downvotes_count: u32,
}

#[derive(Encode, Decode, Default, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PositiveExternalityPostUpdate {
    pub content: Option<Content>,
    pub hidden: Option<bool>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Incentives<T: Config> {
    pub number_of_games: u64,
    pub winner: u64,
    pub loser: u64,
    pub total_stake: u64,
    pub start: WhenDetailsOf<T>,
}

impl<T: Config> Incentives<T> {
    pub fn new(number_of_games: u64, winner: u64, loser: u64, stake: u64) -> Self {
        Incentives {
            number_of_games: number_of_games,
            winner: winner,
            loser: loser,
            total_stake: stake,
            start: new_when_details::<T>(),
        }
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct IncentivesMetaData<T: Config> {
    pub total_number: u64,
    pub disincentive_times: u64,
    pub total_block: BlockNumberOf<T>,
}

impl<T: Config> Default for IncentivesMetaData<T> {
    fn default() -> Self {
        Self {
            total_number: 20,
            disincentive_times: 15, // its 1.5
            total_block: 432000u64.saturated_into::<BlockNumberOf<T>>(), // 30 days = (24*60*60)/6 * 30
        }
    }
}
