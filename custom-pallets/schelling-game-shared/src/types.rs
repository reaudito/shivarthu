use super::*;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Period {
	Evidence,  // Evidence can be submitted. This is also when drawing has to take place.
	Staking, // Stake sum trees can be updated. Pass after `minStakingTime` passes and there is at least one dispute without jurors.
	Drawing, // Jurors can be drawn. Pass after all disputes have jurors or `maxDrawingTime` passes.
	Commit,  // Jurors commit a hashed vote. This is skipped for courts without hidden votes.
	Vote,    // Jurors reveal/cast their vote depending on whether the court has hidden votes or not.
	Appeal,  // The dispute can be appealed.
	Execution, // Tokens are redistributed and the ruling is executed.
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SchellingGameType {
	ProfileApproval,
	ProfileScore,
	ProjectReview,
	PriceDiscovery,
	PositiveExternality,
	DepartmentScore,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PhaseData<T: Config> {
	pub evidence_length: BlockNumberOf<T>,
	pub end_of_staking_time: BlockNumberOf<T>,
	pub staking_length: BlockNumberOf<T>,
	pub drawing_length: BlockNumberOf<T>,
	pub commit_length: BlockNumberOf<T>,
	pub vote_length: BlockNumberOf<T>,
	pub appeal_length: BlockNumberOf<T>,
	pub max_draws: u64,
	pub min_number_juror_staked: u64,
	pub min_juror_stake: BalanceOf<T>,
	pub juror_incentives: (u64, u64), // (looser burn, winner mint)
}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub struct StakingTime<BlockNumber> {
// 	pub min_short_block_length: BlockNumber,
// 	pub min_long_block_length: BlockNumber,
// }

// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub struct DrawJurorsLimit {
// 	pub max_draws: u64,
// 	pub max_draws_appeal: u64,
// }

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum VoteStatus {
	Commited,
	Revealed,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CommitVote {
	pub commit: [u8; 32],
	pub votestatus: VoteStatus,
	pub revealed_vote: Option<RevealedVote>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum RevealedVote {
	Yes,
	No,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum WinningDecision {
	WinnerYes,
	WinnerNo,
	Draw,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum JurorGameResult {
	Won,
	Lost,
	Draw,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScoreCommitVote {
	pub commit: [u8; 32],
	pub votestatus: VoteStatus,
	pub revealed_vote: Option<i64>,
}

/// RangePoint enum to determine whether score values are from
/// 1) ZeroToTen: 0 to 10
/// 2) MinusTenToPlusTen: -10 to +10
/// 3) ZeroToFive: 0 to 5
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum RangePoint {
	ZeroToTen,
	MinusTenToPlusTen,
	ZeroToFive,
}
