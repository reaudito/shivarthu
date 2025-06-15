use crate::conviction::Conviction;
use codec::Decode;
use codec::Encode;
use codec::MaxEncodedLen;
use pallet_support::Content;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Amount of votes and capital placed in delegation for an account.
#[derive(
    Encode, Decode, Default, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen,
)]
pub struct Delegations<Balance> {
    /// The number of votes (this is post-conviction).
    pub votes: Balance,
    /// The amount of raw capital, used for the support.
    pub capital: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct Vote<Balance> {
    pub aye: bool,
    pub conviction: Conviction,
    pub balance: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct VoteRecord<Balance, BlockNumber> {
    pub vote: Vote<Balance>,
    pub expiry: BlockNumber, // expiry block for lock
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum SpenderCategory {
    BigSpender,
    MediumSpender,
    SmallSpender,
    BigTipper,
    SmallTipper,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum FundingStatus {
    Active,
    Finalized,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct FundingInfo<Balance, Moment> {
    /// Optional amount to be released if approved
    pub amount: Option<Balance>,

    /// The group this funding belongs to
    pub group_id: u64,

    /// Start time of the vote
    pub vote_start: Moment,

    /// Status of the funding
    pub status: FundingStatus,

    /// Content associated with the proposal
    pub content: Content,

    /// Stake required from proposer based on their category
    pub stake_amount: Balance,

    /// Conviction-based tally for approval
    pub conviction_tally: (Balance, Balance), // (aye_total, nay_total)
}
