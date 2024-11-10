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
pub mod extras;
pub mod weights;
pub use weights::*;

pub type DepartmentId = u64;
use sp_runtime::AccountId32;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

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
	#[pallet::getter(fn kyc_accounts)]
	pub type KYCAccounts<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentId, Vec<(T::AccountId, [u8; 64])>>;

	#[pallet::storage]
	#[pallet::getter(fn hash_by_range)]
	pub type HashByRange<T: Config> =
		StorageMap<_, Blake2_128Concat, (DepartmentId, u32, u32), [u8; 32]>;

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
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		InvalidLength,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn calculate_hash(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// let account_id_bytes: [u8; 32] = who.encode();
			let current_hash: [u8; 32] = [0; 32];

			// let account_id: AccountId32 = AccountId32::new(account_id_bytes);

			let hash = Self::update_hash_incrementally(current_hash, who.encode());

			// println!("hash {:?}", hash);

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
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
