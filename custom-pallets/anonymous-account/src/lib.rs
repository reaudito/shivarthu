#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod extras;
pub mod weights;
pub use weights::*;

pub type DepartmentId = u64;

use frame_support::pallet_prelude::DispatchError;
use sp_std::collections::btree_set::BTreeSet;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;

	#[pallet::pallet]
	#[pallet::without_storage_info]
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

	#[pallet::storage]
	#[pallet::getter(fn kyc_account_ids)]
	pub type KYCAccountIds<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentId, BTreeSet<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn kyc_accounts)]
	pub type KYCAccounts<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentId, Vec<(T::AccountId, [u8; 64])>>; // Account Id, signature
	#[pallet::storage]
	#[pallet::getter(fn kyc_hash)]
	pub type KYCHashes<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, DepartmentId, Blake2_128Concat, u32, [u8; 32]>;

	#[pallet::type_value]
	pub fn DefaultSliceRange() -> u32 {
		100
	}

	#[pallet::storage]
	#[pallet::getter(fn slice_range)]
	pub type SliceRange<T: Config> = StorageValue<_, u32, ValueQuery, DefaultSliceRange>;

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

		EncodeHash {
			encode: Vec<u8>,
			hash: [u8; 32],
			account_id: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		InvalidLength,
		AccountAlreadyAdded,
		NoAccounts,
		IncompleteSlice,
		HashNotFound,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_kyc_account(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			signature: [u8; 64],
		) -> DispatchResult {
			// ToDo! Check who has kyc.
			// Ensure the caller is a signed origin (authenticated user).
			let who = ensure_signed(origin)?;

			// Check if the caller (who) has already completed KYC for this department.
			KYCAccountIds::<T>::try_mutate(department_id, |account_ids| {
				let account_ids = account_ids.get_or_insert_with(BTreeSet::new);

				// Ensure the account is not already in the KYC list.
				ensure!(!account_ids.contains(&who), Error::<T>::AccountAlreadyAdded);

				// Add the account ID to the set for quick lookup.
				account_ids.insert(who.clone());

				// Add the full account information to the main storage.
				KYCAccounts::<T>::try_mutate(department_id, |accounts| {
					let accounts = accounts.get_or_insert_with(Vec::new);
					accounts.push((who, signature));
					Ok(())
				})
			})
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn calculate_slice_hash(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			slice_number: u32,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let slice_range = Self::slice_range() as usize;
			let start_index = (slice_number as usize) * slice_range;
			let end_index = start_index + slice_range;

			// Retrieve accounts and ensure the slice exists
			let accounts = KYCAccounts::<T>::get(department_id).ok_or(Error::<T>::NoAccounts)?;
			ensure!(accounts.len() >= end_index, Error::<T>::IncompleteSlice);

			// Check if the hash for this slice already exists
			if KYCHashes::<T>::contains_key(department_id, slice_number) {
				return Ok(()); // Hash already exists, no need to recalculate
			}

			// Process accounts in the specified slice
			let slice = &accounts[start_index..end_index];
			let hash = Self::calculate_hash_for_accounts(&slice);

			// let mut current_hash = [0; 32]; // Initial hash for the slice

			// for (account_id, _) in slice {
			// 	let encoded_id = account_id.encode();
			// 	current_hash = Self::update_hash_incrementally(current_hash, encoded_id);
			// }

			// Store the computed hash in storage
			KYCHashes::<T>::insert(department_id, slice_number, hash);

			Ok(())
		}

		#[pallet::call_index(50)]
		#[pallet::weight(0)]
		pub fn calculate_hash(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let current_hash: [u8; 32] = [0; 32];
			let encode = who.clone().encode();

			let hash = Self::update_hash_incrementally(current_hash, who.encode());

			Self::deposit_event(Event::EncodeHash { encode, hash, account_id: who });

			// println!("hash {:?}", hash);

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(51)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
