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
pub mod types;

use crate::types::{SortitionSumTree, SumTreeName};
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};
use trait_sortition_sum_game::SortitionSumGameLink;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type BlockNumberOf<T> = BlockNumberFor<T>;
type SumTreeNameType<T> = SumTreeName<AccountIdOf<T>, BlockNumberOf<T>>;

#[frame_support::pallet]
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

	#[pallet::storage]
	#[pallet::getter(fn sortition_sum_trees)]
	pub type SortitionSumTrees<T> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, SortitionSumTree<AccountIdOf<T>>>;

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		KMustGreaterThanOne,
		TreeAlreadyExists,
		TreeDoesnotExist,
	}
}
