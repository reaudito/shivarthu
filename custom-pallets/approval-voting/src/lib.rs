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
use trait_shared_storage::SharedStorageLink;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

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
    pub trait Config: frame_system::Config + pallet_shared_storage::Config {
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
        Vec<T::AccountId>,
        ValueQuery,
    >;

    /// Stores vote counts for each candidate in a group
    #[pallet::storage]
    #[pallet::getter(fn total_votes_by_group)]
    pub type TotalVotesByGroup<T: Config> =
        StorageMap<_, Twox64Concat, u64, BTreeMap<T::AccountId, u32>, ValueQuery>;

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

            // ensure!(
            //     T::SharedStorageSource::is_member_in_group_district_and_specialization(
            //         group_id,
            //         who.clone()
            //     )?,
            //     Error::<T>::NotMemberOfRequiredDepartments
            // );

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
            candidates: Vec<T::AccountId>,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            ensure!(
                !VotesByGroup::<T>::contains_key(group_id, &voter),
                Error::<T>::AlreadyVoted
            );

            // ensure!(
            //     T::SharedStorageSource::is_member_in_group_district_and_specialization(
            //         group_id,
            //         voter.clone()
            //     )?,
            //     Error::<T>::NotMemberOfRequiredDepartments
            // );

            let candidate_list = CandidatesByGroup::<T>::get(group_id);
            ensure!(
                !candidate_list.is_empty(),
                Error::<T>::NoCandidatesAvailable
            );

            let mut total_votes = TotalVotesByGroup::<T>::get(group_id);

            for candidate in &candidates {
                match candidate_list.binary_search(candidate) {
                    Ok(_) => {
                        *total_votes.entry(candidate.clone()).or_insert(0) += 1;
                    }
                    Err(_) => return Err(Error::<T>::NoSuchCandidate.into()),
                }
            }

            TotalVotesByGroup::<T>::insert(group_id, total_votes);
            VotesByGroup::<T>::insert(group_id, &voter, candidates);

            Self::deposit_event(Event::VoteCast { user: voter });

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
}
