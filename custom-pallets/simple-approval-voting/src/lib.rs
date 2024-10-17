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

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Storage to keep track of all candidates
	#[pallet::storage]
	#[pallet::getter(fn candidates)]
	pub type Candidates<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn votes)]
	pub type Votes<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32, ValueQuery>;

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
		pub fn vote(
			origin: OriginFor<T>,
			approved_candidates: Vec<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure the candidates exist and increment votes
			for candidate in approved_candidates.iter() {
				ensure!(Candidates::<T>::get().contains(candidate), Error::<T>::NoSuchCandidate);
				Votes::<T>::mutate(candidate, |vote_count| *vote_count += 1);
			}

			// Emit an event for the vote
			Self::deposit_event(Event::VoteCast { user: who });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn announce_winner(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Find the candidate with the highest votes
			let mut winner: Option<T::AccountId> = None;
			let mut max_votes = 0;

			for candidate in Candidates::<T>::get().iter() {
				let votes = Votes::<T>::get(candidate);
				if votes > max_votes {
					max_votes = votes;
					winner = Some(candidate.clone());
				}
			}

			if let Some(winner) = winner {
				// Emit the winner announcement event
				Self::deposit_event(Event::WinnerAnnounced { winner });
			}

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn find_top_candidates(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Collect all candidates and their votes
			let mut candidate_votes: Vec<(T::AccountId, u32)> = Candidates::<T>::get()
				.into_iter()
				.map(|candidate| {
					let votes = Votes::<T>::get(&candidate);
					(candidate, votes)
				})
				.collect();

			// Sort candidates by votes in descending order
			candidate_votes.sort_by(|a, b| b.1.cmp(&a.1));

			// Take the top 5 candidates
			let top_candidates = candidate_votes.into_iter().take(5).collect::<Vec<_>>();

			// Emit an event with the top 5 candidates
			Self::deposit_event(Event::TopCandidates { top_candidates: top_candidates.clone() });

			Ok(())
		}
	}
}
