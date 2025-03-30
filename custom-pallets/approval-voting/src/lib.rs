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

			// Ensure candidate is not already added
			ensure!(!Candidates::<T>::get().contains(&candidate), Error::<T>::CandidateExists);

			// Add the candidate
			Candidates::<T>::mutate(|candidates| candidates.push(candidate.clone()));

			// Emit an event
			Self::deposit_event(Event::CandidateAdded { candidate });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn vote(origin: OriginFor<T>, candidates: Vec<T::AccountId>) -> DispatchResult {
			let voter = ensure_signed(origin)?;
			ensure!(!Votes::<T>::contains_key(&voter), Error::<T>::AlreadyVoted);

			let mut total_votes = TotalVotes::<T>::get();
			for candidate in &candidates {
				ensure!(Candidates::<T>::get().contains(candidate), Error::<T>::NoSuchCandidate);
				*total_votes.entry(candidate.clone()).or_insert(0) += 1;
			}

			TotalVotes::<T>::put(total_votes);
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
