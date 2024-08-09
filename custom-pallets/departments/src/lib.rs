//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
pub mod extras;
pub mod types;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use types::{DepartmentDetails, FIRST_DEPARTMENT_ID};
type DepartmentId = u64;
use pallet_support::{
	ensure_content_is_valid, new_who_and_when, remove_from_vec, Content, WhoAndWhen, WhoAndWhenOf,
};

use sp_std::vec;
use sp_std::vec::Vec;
// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	/// A storage item for this pallet.
	///
	/// In this template, we are declaring a storage item called `Something` that stores a single
	/// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
	#[pallet::storage]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::type_value]
	pub fn DefaultForNextDepartmentId() -> DepartmentId {
		FIRST_DEPARTMENT_ID
	}

	#[pallet::storage]
	#[pallet::getter(fn next_department_id)]
	pub type NextDepartmentId<T: Config> =
		StorageValue<_, DepartmentId, ValueQuery, DefaultForNextDepartmentId>;

	#[pallet::storage]
	#[pallet::getter(fn departments)]
	pub type Departments<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentId, DepartmentDetails<T>>; // Peer account id => Peer Profile Hash

	#[pallet::storage]
	#[pallet::getter(fn department_accounts)]
	pub type DepartmentAccounts<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentId, Vec<T::AccountId>>;
	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		DepartmentCreated { account: T::AccountId, department_id: DepartmentId },
		MemberAdded { new_member: T::AccountId, department_id: DepartmentId },
		MemberRemoved { remove_member: T::AccountId, department_id: DepartmentId },
		AdminChanged { admin_changed: T::AccountId, department_id: DepartmentId },
	}

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
		DepartmentDontExists,
		NotAdmin,
		AccountAlreadyExits,
		AlreadyMember,
		NotMember,
		NoAccounts,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a department
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_department(origin: OriginFor<T>, content: Content) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let new_department_id = Self::next_department_id();

			let new_department: DepartmentDetails<T> =
				DepartmentDetails::new(new_department_id, content, who.clone());

			Departments::insert(new_department_id, new_department);

			NextDepartmentId::<T>::mutate(|n| {
				*n += 1;
			});

			Self::deposit_event(Event::DepartmentCreated {
				account: who,
				department_id: new_department_id,
			});

			Ok(())
		}

		/// Add member to department
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn add_member_to_department(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			new_member: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::check_member_is_admin(who, department_id)?;

			match <DepartmentAccounts<T>>::get(department_id) {
				Some(mut accounts) => match accounts.binary_search(&new_member) {
					Ok(_) => Err(Error::<T>::AlreadyMember)?,
					Err(index) => {
						accounts.insert(index, new_member.clone());
						<DepartmentAccounts<T>>::mutate(&department_id, |account_option| {
							*account_option = Some(accounts);
						});
						Self::deposit_event(Event::MemberAdded { new_member, department_id });
					},
				},
				None => {
					<DepartmentAccounts<T>>::insert(department_id, vec![new_member.clone()]);
					Self::deposit_event(Event::MemberAdded { new_member, department_id });
				},
			}

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn remove_member_from_department(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			remove_member: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::check_member_is_admin(who, department_id)?;

			match <DepartmentAccounts<T>>::get(department_id) {
				Some(mut accounts) => match accounts.binary_search(&remove_member) {
					Ok(index) => {
						accounts.remove(index);
						<DepartmentAccounts<T>>::mutate(&department_id, |account_option| {
							*account_option = Some(accounts);
						});
						Self::deposit_event(Event::MemberRemoved { remove_member, department_id });
					},
					Err(_) => Err(Error::<T>::NotMember)?,
				},
				None => Err(Error::<T>::NoAccounts)?,
			}

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn change_the_admin(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			new_admin: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match <Departments<T>>::get(department_id) {
				Some(mut department) => {
					let admin = department.department_admin;
					ensure!(admin == who, Error::<T>::NotAdmin);
					department.department_admin = new_admin.clone();

					<Departments<T>>::mutate(&department_id, |department_option| {
						*department_option = Some(department);
					});

					Self::deposit_event(Event::AdminChanged {
						admin_changed: new_admin,
						department_id,
					});
				},
				None => Err(Error::<T>::DepartmentDontExists)?,
			}

			Ok(())
		}
	}
}
