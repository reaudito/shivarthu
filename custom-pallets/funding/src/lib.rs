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

pub mod types;

use crate::types::{FundingProposal, MajorityApproval, MajorityType};
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::sp_runtime::Saturating;
use frame_support::traits::{
    Currency, ExistenceRequirement, Get, OnUnbalanced, ReservableCurrency, WithdrawReasons,
};
use frame_system::pallet_prelude::*;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
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

        type Currency: ReservableCurrency<Self::AccountId>;
    }

    /// A storage item for this pallet.
    ///
    /// In this template, we are declaring a storage item called `Something` that stores a single
    /// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
    ///

    #[pallet::storage]
    #[pallet::getter(fn funding_proposals)]
    pub type PendingFundingProposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64, // proposal_id
        FundingProposal<BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn group_funding)]
    pub type GroupFunding<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, BalanceOf<T>, ValueQuery>;

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
        /// A user has successfully set a new value.
        SomethingStored {
            /// The new value set.
            something: u32,
            /// The account who set the new value.
            who: T::AccountId,
        },
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
        InsufficientFunds,
        MajorityNotReached,
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
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn propose_distribution(
            origin: OriginFor<T>,
            group_id: u64,
            beneficiary: T::AccountId,
            amount: BalanceOf<T>,
            majority_type: MajorityType,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            let total_funding = GroupFunding::<T>::get(group_id);
            ensure!(total_funding >= amount, Error::<T>::InsufficientFunds);

            // Get top scorers
            // let top_voters = T::ScoreVotingSource::get_top_members(group_id, 5);

            // let mut approval = MajorityApproval::new();
            // for (voter, _) in &top_voters {
            //     let vote =
            //         T::ScoreVotingSource::get_funding_approval_vote(voter, &beneficiary, group_id);
            //     approval.vote(vote);
            // }

            // ensure!(
            //     approval.is_passed(top_voters.len(), majority_type),
            //     Error::<T>::MajorityNotReached
            // );

            // Transfer funds
            // GroupFunding::<T>::mutate(group_id, |f| *f -= amount);
            T::Currency::deposit_into_existing(&beneficiary, amount)?;

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn now() -> u64 {
        pallet_timestamp::Pallet::<T>::get().saturated_into::<u64>()
    }

    pub fn finalize_funding_proposals() -> DispatchResult {
        let now = Self::now();
        let one_month_secs = 30 * 24 * 60 * 60;

        // for (proposal_id, proposal) in PendingFundingProposals::<T>::iter() {
        //     if now.saturating_sub(proposal.created_at) >= one_month_secs {
        //         let approved = T::ConvictionVotingSource::is_proposal_approved(proposal_id)?;

        //         if approved {
        //             GroupFunding::<T>::mutate(proposal.group_id, |f| {
        //                 *f += proposal.amount;
        //             });
        //         }

        //         PendingFundingProposals::<T>::remove(proposal_id);
        //     }
        // }

        Ok(())
    }
}
