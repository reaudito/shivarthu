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

pub mod conviction;
pub mod types;

use crate::conviction::Conviction;
use crate::types::{FundingInfo, FundingStatus, SpenderCategory, Vote, VoteRecord};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::traits::LockIdentifier;
use frame_support::traits::{LockableCurrency, ReservableCurrency};

use frame_support::traits::{Currency, ExistenceRequirement, Get, OnUnbalanced, WithdrawReasons};

use pallet_support::Content;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_runtime::Saturating;
use sp_std::collections::btree_map::BTreeMap;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
pub const UNITS: u64 = 1_000_000_000_000;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use sp_runtime::print;

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
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>
            + ReservableCurrency<Self::AccountId>;
        type MaxLockId: Get<LockIdentifier>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn spendable_balance)]
    pub type SpendableBalance<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_funding_number)]
    pub type NextFundingNumber<T: Config> = StorageValue<_, u32, ValueQuery, ConstU32<0>>;

    #[pallet::storage]
    #[pallet::getter(fn funding_info)]
    pub type FundingInfoStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // funding_id
        FundingInfo<BalanceOf<T>, BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn funding_votes)]
    pub type FundingVotes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // funding_id
        BTreeMap<T::AccountId, VoteRecord<BalanceOf<T>, BlockNumberFor<T>>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn funding_tally)]
    pub type FundingTally<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,                          // funding_id
        (BalanceOf<T>, BalanceOf<T>), // (aye_total, nay_total)
        ValueQuery,
    >;

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
        Voted {
            group_id: u64,
            voter: T::AccountId,
            conviction: Conviction,
            aye: bool,
            weight: BalanceOf<T>,
            capital: BalanceOf<T>,
        },
        Unlocked {
            group_id: u64,
            voter: T::AccountId,
        },
        FundingProposed {
            funding_id: u32,
            proposer: T::AccountId,
            group_id: u64,
            amount: BalanceOf<T>,
            stake: BalanceOf<T>,
            category: SpenderCategory,
        },
        FundingFinalized {
            funding_id: u32,
            group_id: u64,
            approved: bool,
            total_ayes: BalanceOf<T>,
            total_nays: BalanceOf<T>,
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
        ZeroBalance,
        NoVoteFound,
        VoteStillLocked,
        FundingAskedMore,
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
        /// An example dispatchable that takes a single u32 value as a parameter, writes the value
        /// to storage and emits an event.
        ///
        /// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
        /// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>

        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn propose_funding(
            origin: OriginFor<T>,
            group_id: u64,
            amount: BalanceOf<T>,
            content: Content,
            category: SpenderCategory,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            let min_stake = Self::min_stake(&category);
            ensure!(
                T::Currency::free_balance(&proposer) >= min_stake,
                Error::<T>::ZeroBalance
            );

            // Reserve stake
            T::Currency::reserve(&proposer, min_stake)?;

            let max_funding = Self::max_funding(&category);

            ensure!(amount <= max_funding, Error::<T>::FundingAskedMore);

            let now = <frame_system::Pallet<T>>::block_number();
            let funding_id = NextFundingNumber::<T>::get();

            let info = FundingInfo {
                amount: Some(amount),
                group_id,
                vote_start: now,
                status: FundingStatus::Active,
                content,
                stake_amount: min_stake,
                conviction_tally: (Zero::zero(), Zero::zero()),
            };

            FundingInfoStorage::<T>::insert(funding_id, &info);
            NextFundingNumber::<T>::put(funding_id + 1);

            Self::deposit_event(Event::FundingProposed {
                funding_id,
                proposer,
                group_id,
                amount,
                stake: min_stake,
                category,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn vote(
            origin: OriginFor<T>,
            funding_id: u32,
            aye: bool,
            conviction: Conviction,
            balance: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!balance.is_zero(), Error::<T>::ZeroBalance);

            let info = FundingInfoStorage::<T>::get(funding_id).ok_or(Error::<T>::NoVoteFound)?;
            ensure!(
                info.status == FundingStatus::Active,
                Error::<T>::VoteStillLocked
            );

            let new_vote = Vote {
                aye,
                conviction,
                balance,
            };
            let new_weighted = conviction.votes(balance).votes;

            let mut votes = FundingVotes::<T>::get(funding_id);

            if let Some(prev_record) = votes.get(&who) {
                let prev = &prev_record.vote;
                let prev_weight = prev.conviction.votes(prev.balance).votes;

                FundingTally::<T>::mutate(funding_id, |(ayes, nays)| {
                    if prev.aye {
                        *ayes = ayes.saturating_sub(prev_weight);
                    } else {
                        *nays = nays.saturating_sub(prev_weight);
                    }
                });
            }

            T::Currency::set_lock(T::MaxLockId::get(), &who, balance, WithdrawReasons::all());

            let expiry = <frame_system::Pallet<T>>::block_number()
                + BlockNumberFor::<T>::from(conviction.lock_periods());

            votes.insert(
                who.clone(),
                VoteRecord {
                    vote: new_vote,
                    expiry,
                },
            );
            FundingVotes::<T>::insert(funding_id, votes);

            FundingTally::<T>::mutate(funding_id, |(ayes, nays)| {
                if aye {
                    *ayes += new_weighted;
                } else {
                    *nays += new_weighted;
                }
            });

            Self::deposit_event(Event::Voted {
                group_id: info.group_id,
                voter: who,
                conviction,
                aye,
                weight: new_weighted,
                capital: balance,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn unlock(origin: OriginFor<T>, funding_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let votes = FundingVotes::<T>::get(funding_id);
            let vote_record = votes.get(&who).ok_or(Error::<T>::NoVoteFound)?;
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now >= vote_record.expiry, Error::<T>::VoteStillLocked);

            T::Currency::remove_lock(T::MaxLockId::get(), &who);

            let mut updated_votes = votes;
            updated_votes.remove(&who);
            FundingVotes::<T>::insert(funding_id, updated_votes);

            Self::deposit_event(Event::Unlocked {
                group_id: FundingInfoStorage::<T>::get(funding_id)
                    .map(|i| i.group_id)
                    .unwrap_or_default(),
                voter: who,
            });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn finalize_vote(origin: OriginFor<T>, funding_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?; // Optionally use `who`, or omit if not needed

            let mut info =
                FundingInfoStorage::<T>::get(funding_id).ok_or(Error::<T>::NoVoteFound)?;
            ensure!(
                info.status == FundingStatus::Active,
                Error::<T>::VoteStillLocked
            );

            let now = <frame_system::Pallet<T>>::block_number();
            let duration: BlockNumberFor<T> = Self::u64_to_block_saturated(30 * 24 * 60 * 60 / 6);
            let end_block = info.vote_start + duration;

            ensure!(now >= end_block, Error::<T>::VoteStillLocked);

            let (ayes, nays) = FundingTally::<T>::get(funding_id);
            let total_votes = ayes + nays;

            let group_id = info.group_id;
            let approved = ayes > nays && !total_votes.is_zero();

            if approved {
                if let Some(amount) = info.amount {
                    SpendableBalance::<T>::mutate(group_id, |balance| *balance += amount);
                }
            }

            info.status = FundingStatus::Finalized;
            FundingInfoStorage::<T>::insert(funding_id, info);

            Self::deposit_event(Event::FundingFinalized {
                funding_id,
                group_id,
                approved,
                total_ayes: ayes,
                total_nays: nays,
            });

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn min_stake(category: &SpenderCategory) -> BalanceOf<T> {
        match category {
            SpenderCategory::BigSpender => Self::u64_to_balance_saturated(100 * UNITS),
            SpenderCategory::MediumSpender => Self::u64_to_balance_saturated(50 * UNITS),
            SpenderCategory::SmallSpender => Self::u64_to_balance_saturated(25 * UNITS),
            SpenderCategory::BigTipper => Self::u64_to_balance_saturated(20 * UNITS),
            SpenderCategory::SmallTipper => Self::u64_to_balance_saturated(10 * UNITS),
        }
    }

    pub fn max_funding(category: &SpenderCategory) -> BalanceOf<T> {
        match category {
            SpenderCategory::BigSpender => Self::u64_to_balance_saturated(100_000 * UNITS),
            SpenderCategory::MediumSpender => Self::u64_to_balance_saturated(50_000 * UNITS),
            SpenderCategory::SmallSpender => Self::u64_to_balance_saturated(25_000 * UNITS),
            SpenderCategory::BigTipper => Self::u64_to_balance_saturated(75_000 * UNITS),
            SpenderCategory::SmallTipper => Self::u64_to_balance_saturated(10_000 * UNITS),
        }
    }

    pub fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
        input.saturated_into::<BalanceOf<T>>()
    }

    pub fn u64_to_block_saturated(input: u64) -> BlockNumberFor<T> {
        input.saturated_into::<BlockNumberFor<T>>()
    }
}
