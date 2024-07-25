use super::*;
use frame_support::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use scale_info::TypeInfo;

pub const DEPARTMENT_REQUIRED_FUND_ID: DepartmentRequiredFundId = 1;

pub const TIME_FOR_STAKING_FUNDING_STATUS_FAILED: u64 = (3 * 30 * 24 * 60 * 60) / 6; // 3 months time

pub const TIME_FOR_STAKING_FUNDING_STATUS_PASSED: u64 = (6 * 30 * 24 * 60 * 60) / 6; // 6 months time

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum TippingName {
    SmallTipper,
    BigTipper,
    SmallSpender,
    MediumSpender,
    BigSpender,
}

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TippingValue<Balance> {
    pub max_tipping_value: Balance,
    pub stake_required: Balance,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct DepartmentRequiredFund<T: Config> {
    pub created: WhoAndWhenOf<T>,
    pub department_required_fund_id: DepartmentRequiredFundId,
    pub department_id: DepartmentId,
    pub content: Content,
    pub tipping_name: TippingName,
    pub funding_needed: BalanceOf<T>,
    pub creator: T::AccountId,
}

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum FundingStatus {
    Processing,
    Success,
    Failed,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct DepartmentFundingStatus<BlockNumber, FundingStatus> {
    pub block_number: BlockNumber,
    pub status: FundingStatus,
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
