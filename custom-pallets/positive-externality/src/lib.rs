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

mod extras;
pub mod types;
pub use types::{Post, FIRST_POST_ID};

use frame_support::pallet_prelude::DispatchError;
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::Saturating;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{dispatch::DispatchResult, ensure};
use frame_support::{
    traits::{
        Currency, ExistenceRequirement, Get, OnUnbalanced, ReservableCurrency, WithdrawReasons,
    },
    PalletId,
};
use frame_system::pallet_prelude::*;
use pallet_schelling_game_shared::types::{
    JurorGameResult, Period, PhaseData, RangePoint, SchellingGameType, WinningDecision,
};
use pallet_sortition_sum_game::types::SumTreeName;
use pallet_support::{
    ensure_content_is_valid, new_when_details, new_who_and_when, remove_from_vec, Content, PostId,
    WhenDetails, WhenDetailsOf, WhoAndWhen, WhoAndWhenOf,
};
use types::{Incentives, IncentivesMetaData};

use sp_std::prelude::*;
use trait_schelling_game_shared::SchellingGameSharedLink;
use trait_shared_storage::SharedStorageLink;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
pub type BlockNumberOf<T> = BlockNumberFor<T>;
pub type SumTreeNameType<T> = SumTreeName<AccountIdOf<T>, BlockNumberOf<T>>;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_timestamp::Config + pallet_schelling_game_shared::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type representing the weight of this pallet
        type WeightInfo: WeightInfo;

        type SharedStorageSource: SharedStorageLink<AccountId = AccountIdOf<Self>>;
        type SchellingGameSharedSource: SchellingGameSharedLink<
            SumTreeName = SumTreeName<Self::AccountId, BlockNumberOf<Self>>,
            SchellingGameType = SchellingGameType,
            BlockNumber = BlockNumberOf<Self>,
            AccountId = AccountIdOf<Self>,
            Balance = BalanceOf<Self>,
            RangePoint = RangePoint,
            Period = Period,
            PhaseData = PhaseData<Self>,
            WinningDecision = WinningDecision,
            JurorGameResult = JurorGameResult,
        >;
        type Currency: ReservableCurrency<Self::AccountId>;
        /// Handler for the unbalanced increment when rewarding (minting rewards)
        type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;
    }

    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::type_value]
    pub fn DefaultForNextPostId() -> PostId {
        FIRST_POST_ID
    }

    /// The next post id.
    #[pallet::storage]
    #[pallet::getter(fn next_post_id)]
    pub type NextPostId<T: Config> = StorageValue<_, PostId, ValueQuery, DefaultForNextPostId>;

    /// Get the details of a post by its' id.
    #[pallet::storage]
    #[pallet::getter(fn post_by_id)]
    pub type PostById<T: Config> = StorageMap<_, Twox64Concat, PostId, Post<T>>;

    #[pallet::storage]
    #[pallet::getter(fn evidence)]
    pub type Evidence<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<PostId>, ValueQuery>;

    #[pallet::type_value]
    pub fn MinimumStake<T: Config>() -> BalanceOf<T> {
        10000u128.saturated_into::<BalanceOf<T>>()
    }

    #[pallet::storage]
    #[pallet::getter(fn user_stake)]
    pub type StakeBalance<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::type_value]
    pub fn DefaultValidate<T: Config>() -> bool {
        true
    }

    #[pallet::storage]
    #[pallet::getter(fn validate)]
    pub type Validate<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, bool, ValueQuery, DefaultValidate<T>>;

    #[pallet::storage]
    #[pallet::getter(fn validation_block)]
    pub type ValidationBlock<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BlockNumberOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn got_incentives_positive_externality)]
    pub type GotPositiveExternality<T: Config> =
        StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn incentives_count)]
    pub type IncentiveCount<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Incentives<T>>;

    #[pallet::type_value]
    pub fn IncentivesMetaValue<T: Config>() -> IncentivesMetaData<T> {
        IncentivesMetaData::default()
    }

    #[pallet::storage]
    #[pallet::getter(fn incentives_meta)]
    pub type IncentivesMeta<T: Config> =
        StorageValue<_, IncentivesMetaData<T>, ValueQuery, IncentivesMetaValue<T>>;

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
        NotAPostOwner,
        ValidationPositiveExternalityIsOff,
        LessThanMinStake,
        CannotStakeNow,
        ChoiceOutOfRange,
        NotReachedMinimumDecision,
        NoIncentiveCount,
        AlreadyFunded,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_positive_externality_post(
            origin: OriginFor<T>,
            content: Content,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            ensure_content_is_valid(content.clone())?;

            // Citizen approved To comment out in production, citizen approved in added in profile validation

            // T::SharedStorageSource::check_citizen_is_approved_link(creator.clone())?;

            let new_post_id = Self::next_post_id();

            let new_post: Post<T> = Post::new(new_post_id, creator.clone(), content.clone());

            Evidence::<T>::mutate(creator, |ids| ids.push(new_post_id));

            PostById::insert(new_post_id, new_post);
            NextPostId::<T>::mutate(|n| {
                *n += 1;
            });

            // emit event

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn set_validate_positive_externality(
            origin: OriginFor<T>,
            value: bool,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Check user has done kyc

            Validate::<T>::insert(&who, value);
            // emit event
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn apply_staking_period(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::ensure_validation_on_positive_externality(user_to_calculate.clone())?;

            let stake = MinimumStake::<T>::get();

            let _ = <T as pallet::Config>::Currency::withdraw(
                &who,
                stake,
                WithdrawReasons::TRANSFER,
                ExistenceRequirement::AllowDeath,
            )?;

            StakeBalance::<T>::insert(&user_to_calculate, stake);

            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());
            // println!("{:?}", pe_block_number);
            let zero_block_number = Self::u64_to_block_saturated(0);
            let now = <frame_system::Pallet<T>>::block_number();
            let three_month_number = (3 * 30 * 24 * 60 * 60) / 6;
            let three_month_block = Self::u64_to_block_saturated(three_month_number);
            let modulus = now % three_month_block;
            let storage_main_block = now - modulus;
            // println!("now: {:?}", now);
            // println!("three month number{:?}", three_month_number);
            // println!("storage main block {:?}", storage_main_block);
            // println!("pe block number {:?}", pe_block_number);

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate.clone(),
                block_number: storage_main_block.clone(),
            };

            // let game_type = SchellingGameType::PositiveExternality;
            // || pe_block_number == zero_block_number

            if storage_main_block > pe_block_number || pe_block_number == zero_block_number {
                <ValidationBlock<T>>::insert(user_to_calculate.clone(), storage_main_block);
                // check what if called again
                T::SchellingGameSharedSource::set_to_staking_period_pe_link(key.clone(), now)?;
                T::SchellingGameSharedSource::create_tree_helper_link(key, 3)?;

            //  println!("{:?}", data);
            } else {
                return Err(Error::<T>::CannotStakeNow.into());
            }

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn apply_jurors(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::ensure_validation_on_positive_externality(user_to_calculate.clone())?;
            Self::ensure_min_stake_positive_externality(user_to_calculate.clone())?;

            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: pe_block_number.clone(),
            };

            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::apply_jurors_helper_link(key, phase_data, who, stake)?;

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn pass_period(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: pe_block_number.clone(),
            };

            let now = <frame_system::Pallet<T>>::block_number();
            let phase_data = Self::get_phase_data();
            T::SchellingGameSharedSource::change_period_link(key, phase_data, now)?;

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(0)]
        pub fn draw_jurors(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
            iterations: u64,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: pe_block_number.clone(),
            };

            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::draw_jurors_helper_link(key, phase_data, iterations)?;

            Ok(())
        }

        // Unstaking
        // Stop drawn juror to unstake ✔️
        #[pallet::call_index(6)]
        #[pallet::weight(0)]
        pub fn unstaking(origin: OriginFor<T>, user_to_calculate: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: pe_block_number.clone(),
            };

            T::SchellingGameSharedSource::unstaking_helper_link(key, who)?;
            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(0)]
        pub fn commit_vote(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
            vote_commit: [u8; 32],
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: pe_block_number.clone(),
            };

            T::SchellingGameSharedSource::commit_vote_for_score_helper_link(key, who, vote_commit)?;
            Ok(())
        }

        #[pallet::call_index(8)]
        #[pallet::weight(0)]
        pub fn reveal_vote(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
            choice: i64,
            salt: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(choice <= 5 && choice >= 1, Error::<T>::ChoiceOutOfRange);

            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: pe_block_number.clone(),
            };

            T::SchellingGameSharedSource::reveal_vote_score_helper_link(key, who, choice, salt)?;
            Ok(())
        }

        #[pallet::call_index(9)]
        #[pallet::weight(0)]
        pub fn set_new_mean_value(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId, 
        ) -> DispatchResult {
            let _= ensure_signed(origin)?;
            let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: pe_block_number.clone(),
            };

            T::SchellingGameSharedSource::set_new_mean_value(key)?;
            Ok(())
        }

        #[pallet::call_index(10)]
        #[pallet::weight(0)]
        pub fn release_positive_externality_fund(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate.clone(),
                block_number: block_number.clone(),
            };

            let score = T::SchellingGameSharedSource::get_mean_value_link(key.clone())?;

            // println!("Score {:?}", score);

            T::SharedStorageSource::set_positive_externality_link(
                user_to_calculate.clone(),
                score,
            )?;

            let got_incentives_bool = <GotPositiveExternality<T>>::get(key.clone());

            if got_incentives_bool == false {
                <GotPositiveExternality<T>>::insert(key.clone(), true);
                let balance = Self::u64_to_balance_saturated((score as u64) * 1000);

                let r = <T as pallet::Config>::Currency::deposit_into_existing(
                    &user_to_calculate,
                    balance,
                )
                .ok()
                .unwrap();
                <T as pallet::Config>::Reward::on_unbalanced(r);
            } else {
                Err(Error::<T>::AlreadyFunded)?
            }

            Ok(())
        }

        #[pallet::call_index(11)]
        #[pallet::weight(0)]
        pub fn add_incentive_count(
            origin: OriginFor<T>,
            user_to_calculate: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

            let key = SumTreeName::PositiveExternality {
                user_address: user_to_calculate,
                block_number: block_number.clone(),
            };
            let (juror_game_result, stake) =
                T::SchellingGameSharedSource::get_result_of_juror_score(
                    key.clone(),
                    who.clone(),
                    RangePoint::ZeroToFive,
                )?;

            T::SchellingGameSharedSource::add_to_incentives_count(key, who.clone())?;
            let incentive_count_option = <IncentiveCount<T>>::get(&who);
            match incentive_count_option {
                Some(mut incentive) => {
                    match juror_game_result {
                        JurorGameResult::Won => {
                            incentive.number_of_games += 1;
                            incentive.winner += 1;
                            incentive.total_stake += stake;
                        }
                        JurorGameResult::Lost => {
                            incentive.number_of_games += 1;
                            incentive.loser += 1;
                            incentive.total_stake += stake;
                        }

                        JurorGameResult::Draw => {
                            incentive.number_of_games += 1;
                            incentive.total_stake += stake;
                        }
                    };
                    <IncentiveCount<T>>::mutate(&who, |incentive_option| {
                        *incentive_option = Some(incentive);
                    });
                }
                None => {
                    let mut winner = 0;
                    let mut loser = 0;
                    match juror_game_result {
                        JurorGameResult::Won => {
                            winner = 1;
                        }
                        JurorGameResult::Lost => {
                            loser = 1;
                        }
                        JurorGameResult::Draw => {}
                    };
                    let number_of_games = 1;
                    let new_incentives: Incentives<T> =
                        Incentives::new(number_of_games, winner, loser, stake);
                    <IncentiveCount<T>>::insert(&who, new_incentives);
                }
            }

            Ok(())
        }

        // Provide incentives

        #[pallet::call_index(12)]
        #[pallet::weight(0)]
        pub fn get_incentives(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let incentive_meta = <IncentivesMeta<T>>::get();
            let total_games_allowed = incentive_meta.total_number;
            let incentive_count_option = <IncentiveCount<T>>::get(&who);
            match incentive_count_option {
                Some(incentive) => {
                    let total_number_games = incentive.number_of_games;
                    if total_number_games >= total_games_allowed {
                        let new_incentives: Incentives<T> = Incentives::new(0, 0, 0, 0);
                        <IncentiveCount<T>>::mutate(&who, |incentive_option| {
                            *incentive_option = Some(new_incentives);
                        });

                        let total_win = incentive.winner;
                        let total_lost = incentive.loser;

                        // Define multipliers
                        let win_multiplier = 10 * 100;
                        let lost_multiplier = incentive_meta.disincentive_times * 100;

                        // Calculate total_win_incentives and total_lost_incentives
                        let total_win_incentives = total_win.checked_mul(win_multiplier);
                        let total_lost_incentives = total_lost.checked_mul(lost_multiplier);

                        // Calculate total_incentives, handling overflow or negative errors
                        let total_incentives = match (total_win_incentives, total_lost_incentives) {
                            (Some(win), Some(lost)) => win.checked_sub(lost).unwrap_or(0),
                            _ => 0, // If multiplication overflowed, set total_incentives to 0
                        };

                        let mut stake = incentive.total_stake;
                        // Deduct 1% of the stake if total_lost > total_win
                        if total_lost > total_win {
                            let stake_deduction = stake / 100; // 1% of the stake
                            stake = stake.checked_sub(stake_deduction).unwrap_or(stake);
                            // Safe subtraction
                            // println!("Stake deducted by 1%: {}", stake);
                        }

                        let total_fund = stake.checked_add(total_incentives).unwrap_or(0);

                        let balance = Self::u64_to_balance_saturated(total_fund);

                        let r =
                            <T as pallet::Config>::Currency::deposit_into_existing(&who, balance)
                                .ok()
                                .unwrap();
                        <T as pallet::Config>::Reward::on_unbalanced(r);
                        // Provide the incentives
                    } else {
                        Err(Error::<T>::NotReachedMinimumDecision)?
                    }
                }
                None => Err(Error::<T>::NoIncentiveCount)?,
            }
            Ok(())
        }

        // #[pallet::call_index(9)]
        // #[pallet::weight(0)]
        // pub fn get_incentives(
        //     origin: OriginFor<T>,
        //     user_to_calculate: T::AccountId,
        // ) -> DispatchResult {
        //     let _who = ensure_signed(origin)?;
        //     let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

        //     let key = SumTreeName::PositiveExternality {
        //         user_address: user_to_calculate.clone(),
        //         block_number: pe_block_number.clone(),
        //     };

        //     let phase_data = Self::get_phase_data();
        //     T::SchellingGameSharedSource::get_incentives_score_schelling_helper_link(
        //         key.clone(),
        //         phase_data,
        //         RangePoint::ZeroToFive,
        //     )?;

        //     let score = T::SchellingGameSharedSource::get_mean_value_link(key.clone())?;
        //     // println!("Score {:?}", score);
        //     T::SharedStorageSource::set_positive_externality_link(user_to_calculate, score)?;

        //     Ok(())
        // }
    }
}
