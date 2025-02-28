#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod extras;
pub mod types;

use frame_support::traits::BuildGenesisConfig;
use sp_std::prelude::*;
use types::ReputationScore;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type Score = i64;
type DepartmentId = u64;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn approved_citizen_address)]
	pub type ApprovedCitizenAddress<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>; // Its set, add element through binary_search

	#[pallet::storage]
	#[pallet::getter(fn approved_citizen_address_by_department)]
	pub type ApprovedCitizenAddressByDepartment<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentId, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn positive_externality_score)]
	pub type PositiveExternalityScore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Score, ValueQuery>;

	// Keep winning representatives of department in shared storage

	#[pallet::storage]
	#[pallet::getter(fn  reputation_score)]
	pub type ReputationScoreOfAccount<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, ReputationScore>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub approved_citizen_address: Vec<T::AccountId>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { approved_citizen_address: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			<ApprovedCitizenAddress<T>>::put(self.approved_citizen_address.clone());
		}
	}

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
		CitizenNotApproved,
		AlreadyMember,
	}
}
