#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod extras;
mod functions;
mod score_game;
mod share_link;
pub mod types;

use crate::types::{
    CommitVote, JurorGameResult, Period, PhaseData, RangePoint, RevealedVote, SchellingGameType,
    ScoreCommitVote, VoteStatus, WinningDecision,
};
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::traits::Randomness;
use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency};
use frame_system::pallet_prelude::*;
use num_integer::Roots;
use pallet_sortition_sum_game::types::SumTreeName;
use scale_info::prelude::format;
use sp_std::prelude::*;
use trait_sortition_sum_game::SortitionSumGameLink;

pub type BlockNumberOf<T> = BlockNumberFor<T>;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;
type SumTreeNameType<T> = SumTreeName<AccountIdOf<T>, BlockNumberOf<T>>;
type PhaseDataOf<T> = PhaseData<T>;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type SortitionSumGameSource: SortitionSumGameLink<
            SumTreeName = SumTreeName<Self::AccountId, BlockNumberOf<Self>>,
            AccountId = Self::AccountId,
        >;

        type Currency: ReservableCurrency<Self::AccountId>;

        type RandomnessSource: Randomness<Self::Hash, BlockNumberFor<Self>>;

        /// Handler for the unbalanced increment when rewarding (minting rewards)
        type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

        /// Handler for the unbalanced decrement when slashing (burning collateral)
        type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
    }

    #[pallet::storage]
    pub type Nonce<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_period)]
    pub type PeriodName<T> = StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Period>;

    #[pallet::storage]
    #[pallet::getter(fn draws_in_round)]
    pub type DrawsInRound<T> = StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, u64, ValueQuery>; // A counter of draws made in the current round.

    #[pallet::storage]
    #[pallet::getter(fn evidence_start_time)]
    pub type EvidenceStartTime<T> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn staking_start_time)]
    pub type StakingStartTime<T> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn commit_start_time)]
    pub type CommitStartTime<T> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn vote_start_time)]
    pub type VoteStartTime<T> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

    /// Drawn jurors containing account id and stake Vec<(AccountId, Stake)>
    /// Should be stored in sorted order by AccountId
    #[pallet::storage]
    #[pallet::getter(fn  drawn_jurors)]
    pub type DrawnJurors<T: Config> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<(T::AccountId, u64)>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn unstaked_jurors)]
    pub type UnstakedJurors<T: Config> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<T::AccountId>, ValueQuery>;

    /// VoteCommits for Yes or No voting
    #[pallet::storage]
    #[pallet::getter(fn vote_commits)]
    pub type VoteCommits<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        SumTreeNameType<T>,
        Blake2_128Concat,
        T::AccountId,
        CommitVote,
    >;

    /// Vote Commits for Score Schelling
    #[pallet::storage]
    #[pallet::getter(fn vote_commits_score)]
    pub type ScoreVoteCommits<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        SumTreeNameType<T>,
        Blake2_128Concat,
        T::AccountId,
        ScoreCommitVote,
    >;

    /// Reveal values of score schelling game as Vec<i64>
    #[pallet::storage]
    #[pallet::getter(fn reveal_score_values)]
    pub type RevealScoreValues<T: Config> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<i64>, ValueQuery>;

    /// New mean from the reveal values in score schelling game
    /// Improvement: This step will not be required if all jurors incentives are distributed at one time
    #[pallet::storage]
    #[pallet::getter(fn new_mean_reveal_score)]
    pub type IncentiveMeanRevealScore<T: Config> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, i64>;

    /// Decision count for two choices after reveal vote:  (count for 0, count for 1)
    #[pallet::storage]
    #[pallet::getter(fn decision_count)]
    pub type DecisionCount<T> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, (u64, u64), ValueQuery>; // Count for 0, Count for 1

    #[pallet::storage]
    #[pallet::getter(fn juror_incentive_distribution)]
    pub type JurorsIncentiveDistributedAccounts<T: Config> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn incentive_added_to_count)]
    pub type IncentiveAddedToCount<T: Config> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<T::AccountId>, ValueQuery>;

    // #[pallet::storage]
    // #[pallet::getter(fn )]

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored { something: u32, who: T::AccountId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        PeriodExists,
        EvidencePeriodNotOver,
        StakingPeriodNotOver,
        PeriodIsNotEvidence,
        PeriodIsNotNone,
        MaxJurorNotDrawn,
        CommitPeriodNotOver,
        VotePeriodNotOver,
        PeriodDoesNotExists,
        PeriodDontMatch,
        JurorStakeLessThanMin,
        AlreadyStaked,
        MaxDrawExceeded,
        SelectedAsJuror,
        AlreadyUnstaked,
        StakeDoesNotExists,
        JurorDoesNotExists,
        VoteStatusNotCommited,
        NotValidChoice,
        CommitDoesNotMatch,
        CommitDoesNotExists,
        AlreadyGotIncentives,
        AlreadyIncentivesAdded,
        VoteNotRevealed,
        TimeForStakingOver,
        TimeForStakingNotOver,
        NewMeanNotInserted,
    }
}
