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
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	// Storage to keep track of all candidates
	#[pallet::storage]
	#[pallet::getter(fn candidates)]
	pub type Candidates<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn votes)]
	pub type Votes<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_votes)]
	pub type TotalVotes<T: Config> = StorageValue<_, BTreeMap<T::AccountId, u32>, ValueQuery>;

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
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_candidate(origin: OriginFor<T>, candidate: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure the candidate is not already added
			let mut candidates = Candidates::<T>::get();
			match candidates.binary_search(&candidate) {
				// If the search succeeds, the candidate is already present
				Ok(_) => Err(Error::<T>::CandidateExists.into()),
				// If the search fails, the candidate is not present, and we learn the insertion index
				Err(index) => {
					// Insert the candidate at the correct position to maintain sorted order
					candidates.insert(index, candidate.clone());

					// Update the storage with the modified list
					Candidates::<T>::put(candidates);

					// Emit an event
					Self::deposit_event(Event::CandidateAdded { candidate });

					Ok(())
				},
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn vote(origin: OriginFor<T>, candidates: Vec<T::AccountId>) -> DispatchResult {
			let voter = ensure_signed(origin)?;
			ensure!(!Votes::<T>::contains_key(&voter), Error::<T>::AlreadyVoted);

			// Fetch the sorted list of candidates
			let candidate_list = Candidates::<T>::get();
			ensure!(!candidate_list.is_empty(), Error::<T>::NoCandidatesAvailable);

			// Get the total votes storage
			let mut total_votes = TotalVotes::<T>::get();

			for candidate in &candidates {
				// Use binary search to check if the candidate exists
				match candidate_list.binary_search(candidate) {
					Ok(_) => {
						// Candidate exists, increment their vote count
						*total_votes.entry(candidate.clone()).or_insert(0) += 1;
					},
					Err(_) => {
						// Candidate does not exist, return an error
						return Err(Error::<T>::NoSuchCandidate.into());
					},
				}
			}

			// Update the total votes storage
			TotalVotes::<T>::put(total_votes);

			// Record the voter's vote
			Votes::<T>::insert(&voter, candidates.clone());

			// Emit an event for the vote
			Self::deposit_event(Event::VoteCast { user: voter });

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn get_top_n_winners(n: usize) -> Vec<(T::AccountId, u32)> {
		TotalVotes::<T>::get().into_iter().take(n).collect()
	}
}
