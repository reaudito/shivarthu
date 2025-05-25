#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub mod types;
use trait_shared_storage::SharedStorageLink;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::sp_runtime::Saturating;

pub use weights::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::collections::btree_map::BTreeMap;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_timestamp::Config + pallet_shared_storage::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type representing the weight of this pallet
        type WeightInfo: WeightInfo;

        type SharedStorageSource: SharedStorageLink<AccountId = AccountIdOf<Self>>;
    }

    #[pallet::storage]
    #[pallet::getter(fn candidates_by_group)]
    pub type CandidatesByGroup<T: Config> =
        StorageMap<_, Twox64Concat, u64, Vec<T::AccountId>, ValueQuery>;

    /// Stores the votes cast by each user in a group
    #[pallet::storage]
    #[pallet::getter(fn votes_by_group)]
    pub type VotesByGroup<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        u64,
        Twox64Concat,
        T::AccountId,
        BTreeMap<T::AccountId, u8>, // or u32 for a larger score range
        ValueQuery,
    >;

    /// Stores vote counts for each candidate in a group
    #[pallet::storage]
    #[pallet::getter(fn total_votes_by_group)]
    pub type TotalVotesByGroup<T: Config> =
        StorageMap<_, Twox64Concat, u64, BTreeMap<T::AccountId, u32>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn vote_timestamps)]
    pub type VoteTimestamps<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        u64, // group_id
        Twox64Concat,
        T::AccountId,
        T::Moment,
        OptionQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored {
            something: u32,
            who: T::AccountId,
        },
        CandidateAdded {
            candidate: T::AccountId,
        },
        VoteCast {
            user: T::AccountId,
        },
        WinnerAnnounced {
            winner: T::AccountId,
        },
        TopCandidates {
            top_candidates: Vec<(T::AccountId, u32)>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        CandidateExists,
        NoSuchCandidate,
        AlreadyVoted,
        NoCandidatesAvailable,
        NotMemberOfRequiredDepartments,
        ScoreTooHigh,
        ScoreZeroOrLess,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn add_candidate(origin: OriginFor<T>, group_id: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                T::SharedStorageSource::is_member_in_group_district_and_specialization(
                    group_id,
                    who.clone()
                )?,
                Error::<T>::NotMemberOfRequiredDepartments
            );

            // Ensure the candidate is not already added for this group
            let mut candidates = CandidatesByGroup::<T>::get(group_id);
            match candidates.binary_search(&who) {
                Ok(_) => Err(Error::<T>::CandidateExists.into()),
                Err(index) => {
                    candidates.insert(index, who.clone());
                    CandidatesByGroup::<T>::insert(group_id, candidates);

                    Self::deposit_event(Event::CandidateAdded { candidate: who });
                    Ok(())
                }
            }
        }
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn vote(
            origin: OriginFor<T>,
            group_id: u64,
            scores: BTreeMap<T::AccountId, u8>,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            ensure!(
                T::SharedStorageSource::is_member_in_group_district_and_specialization(
                    group_id,
                    voter.clone()
                )?,
                Error::<T>::NotMemberOfRequiredDepartments
            );

            let candidate_list = CandidatesByGroup::<T>::get(group_id);
            ensure!(
                !candidate_list.is_empty(),
                Error::<T>::NoCandidatesAvailable
            );

            // Ensure all candidates are valid and scores are in range [0, 5]
            for (candidate, score) in &scores {
                ensure!(
                    candidate_list.contains(candidate),
                    Error::<T>::NoSuchCandidate
                );
                ensure!(*score <= 5, Error::<T>::ScoreTooHigh);
                ensure!(*score > 0, Error::<T>::ScoreZeroOrLess);
            }

            let mut total_votes = TotalVotesByGroup::<T>::get(group_id);

            // Subtract old scores if any
            let old_scores = VotesByGroup::<T>::get(group_id, &voter);
            for (candidate, old_score) in old_scores {
                if let Some(total) = total_votes.get_mut(&candidate) {
                    *total = total.saturating_sub(old_score as u32);
                    if *total == 0 {
                        total_votes.remove(&candidate);
                    }
                }
            }

            // Apply new scores
            for (candidate, score) in &scores {
                *total_votes.entry(candidate.clone()).or_insert(0) += *score as u32;
            }

            VotesByGroup::<T>::insert(group_id, &voter, scores);
            TotalVotesByGroup::<T>::insert(group_id, total_votes);
            VoteTimestamps::<T>::insert(group_id, &voter, <pallet_timestamp::Pallet<T>>::get());

            Self::deposit_event(Event::VoteCast { user: voter });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn cleanup_old_votes(origin: OriginFor<T>, group_id: u64) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            Self::remove_stale_votes(group_id);
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn get_top_n_winners(group_id: u64, n: usize) -> Vec<(T::AccountId, u32)> {
        TotalVotesByGroup::<T>::get(group_id)
            .into_iter()
            .take(n)
            .collect()
    }

    pub fn remove_stale_votes(group_id: u64) {
        let now = <pallet_timestamp::Pallet<T>>::get();
        let three_months = T::Moment::saturated_from(1000u64 * 60 * 60 * 24 * 90);
        let mut to_remove = vec![];

        for (voter, timestamp) in VoteTimestamps::<T>::iter_prefix(group_id) {
            if now.saturating_sub(timestamp) > three_months {
                to_remove.push(voter);
            }
        }

        if to_remove.is_empty() {
            return;
        }

        let mut total_votes = TotalVotesByGroup::<T>::get(group_id);

        for voter in &to_remove {
            let voted_scores = VotesByGroup::<T>::get(group_id, voter);
            for (candidate, score) in voted_scores {
                if let Some(count) = total_votes.get_mut(&candidate) {
                    *count = count.saturating_sub(score as u32);
                    if *count == 0 {
                        total_votes.remove(&candidate);
                    }
                }
            }

            VotesByGroup::<T>::remove(group_id, voter);
            VoteTimestamps::<T>::remove(group_id, voter);
        }

        TotalVotesByGroup::<T>::insert(group_id, total_votes);
    }
}
