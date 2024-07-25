#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
// One can enhance validation measures by increasing staking power for local residents or individuals with positive externalities—those who contribute to the network for a good cause.
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
mod types;

use frame_support::pallet_prelude::DispatchError;
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{dispatch::DispatchResult, ensure};
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

use frame_support::{
    traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons, OnUnbalanced},
    PalletId,
};
use pallet_schelling_game_shared::types::{Period, PhaseData, RangePoint, SchellingGameType, JurorGameResult};
use pallet_sortition_sum_game::types::SumTreeName;
use pallet_support::{
    ensure_content_is_valid, new_who_and_when, remove_from_vec, Content, PostId, WhoAndWhen, WhenDetailsOf,
    WhoAndWhenOf, new_when_details
};
use trait_schelling_game_shared::SchellingGameSharedLink;
use trait_shared_storage::SharedStorageLink;
pub use types::DEPARTMENT_REQUIRED_FUND_ID;
use types::{
    DepartmentFundingStatus, DepartmentRequiredFund, FundingStatus, TippingName, TippingValue,
};

use types::{Incentives, IncentivesMetaData};


type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
pub type BlockNumberOf<T> = BlockNumberFor<T>;
pub type SumTreeNameType<T> = SumTreeName<AccountIdOf<T>, BlockNumberOf<T>>;
type DepartmentId = u64;
type DepartmentRequiredFundId = u64;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_schelling_game_shared::Config + pallet_timestamp::Config
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
            JurorGameResult = JurorGameResult,
        >;
        type Currency: ReservableCurrency<Self::AccountId>;
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
    pub fn MinimumDepartmentStake<T: Config>() -> BalanceOf<T> {
        10000u128.saturated_into::<BalanceOf<T>>()
    }

    #[pallet::type_value]
    pub fn DefaultForNextDepartmentRequiredFundId() -> DepartmentRequiredFundId {
        DEPARTMENT_REQUIRED_FUND_ID
    }

    #[pallet::storage]
    #[pallet::getter(fn next_department_required_fund_id)]
    pub type NextDepartmentRequiredFundId<T: Config> = StorageValue<
        _,
        DepartmentRequiredFundId,
        ValueQuery,
        DefaultForNextDepartmentRequiredFundId,
    >;

    #[pallet::storage]
    #[pallet::getter(fn get_department_required_funds)]
    pub type DepartmentRequiredFunds<T: Config> =
        StorageMap<_, Blake2_128Concat, DepartmentRequiredFundId, DepartmentRequiredFund<T>>;

    #[pallet::storage]
    #[pallet::getter(fn validate_positive_externality)]
    pub type ValidateDepartmentRequiredFund<T: Config> =
        StorageMap<_, Twox64Concat, DepartmentRequiredFundId, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validation_block)]
    pub type ValidationBlock<T: Config> =
        StorageMap<_, Blake2_128Concat, DepartmentRequiredFundId, BlockNumberOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn department_funding_status)]
    pub type DepartmentFundingStatusForDepartmentId<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        DepartmentId,
        DepartmentFundingStatus<BlockNumberOf<T>, FundingStatus>,
    >;

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
        DepartmentFundCreated {
            account: T::AccountId,
            department_required_fund_id: DepartmentRequiredFundId,
        },
        StakingPeriodStarted {
            department_required_fund_id: DepartmentRequiredFundId,
            block_number: BlockNumberOf<T>,
        },
        ApplyJurors {
            department_required_fund_id: DepartmentRequiredFundId,
            block_number: BlockNumberOf<T>,
            account: T::AccountId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        LessThanMinStake,
        CannotStakeNow,
        ChoiceOutOfRange,
        FundingMoreThanTippingValue,
        DepartmentRequiredFundDontExits,
        BlockDepartmentRequiredFundIdNotExists,
        ValidationForDepartmentRequiredFundIdIsOff,
        FundingStatusProcessing,
        ReapplicationTimeNotReached,
        ConditionDontMatch,
        NotReachedMinimumDecision,
        NoIncentiveCount,
        AlreadyFunded,
    }

    // Check deparment exists, it will done using loose coupling
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_department_required_fund(
            origin: OriginFor<T>,
            department_id: DepartmentId,
            content: Content,
            tipping_name: TippingName,
            funding_needed: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let tipping_value = Self::value_of_tipping_name(tipping_name);
            let max_tipping_value = tipping_value.max_tipping_value;
            let stake_required = tipping_value.stake_required;
            let new_department_fund_id = Self::next_department_required_fund_id();
            let new_department_fund: DepartmentRequiredFund<T> = DepartmentRequiredFund::new(
                new_department_fund_id,
                department_id,
                content,
                tipping_name,
                funding_needed,
                who.clone(),
            );
            ensure!(
                funding_needed <= max_tipping_value,
                Error::<T>::FundingMoreThanTippingValue
            );
            // Check user has done kyc
            let _ = <T as pallet::Config>::Currency::withdraw(
                &who,
                stake_required,
                WithdrawReasons::TRANSFER,
                ExistenceRequirement::AllowDeath,
            )?;
            DepartmentRequiredFunds::insert(new_department_fund_id, new_department_fund);
            NextDepartmentRequiredFundId::<T>::mutate(|n| {
                *n += 1;
            });

            Self::deposit_event(Event::DepartmentFundCreated {
                account: who,
                department_required_fund_id: new_department_fund_id,
            });
            Ok(())
        }

        // Check update and discussion time over, only project creator can apply staking period
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn apply_staking_period(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::ensure_validation_to_do(department_required_fund_id)?;
            let department_id = Self::get_department_id_from_department_required_fund_id(
                department_required_fund_id,
            )?;
            let department_funding_status = Self::ensure_can_stake_using_status(department_id)?;
            DepartmentFundingStatusForDepartmentId::<T>::insert(
                department_id,
                department_funding_status,
            );
            let now = <frame_system::Pallet<T>>::block_number();
            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: now.clone(),
            };
            ValidationBlock::<T>::insert(department_required_fund_id, now.clone());
            T::SchellingGameSharedSource::set_to_staking_period_pe_link(key.clone(), now.clone())?;
            T::SchellingGameSharedSource::create_tree_helper_link(key, 3)?;

            Self::deposit_event(Event::StakingPeriodStarted {
                department_required_fund_id,
                block_number: now,
            });

            Ok(())
        }

        // // Check update and discussion time over, only project creator can apply staking period
        // #[pallet::call_index(1)]
        // #[pallet::weight(0)]
        // pub fn apply_staking_period(origin: OriginFor<T>, department_required_fund_id: DepartmentRequiredFundId) -> DispatchResult {
        // 	let who = ensure_signed(origin)?;

        // 	Self::ensure_user_is_project_creator_and_project_exists(project_id, who)?;
        // 	Self::ensure_staking_period_set_once_project_id(project_id)?;

        // 	let now = <frame_system::Pallet<T>>::block_number();

        // 	let key = SumTreeName::ProjectTips { project_id, block_number: now.clone() };

        // 	<ValidationProjectBlock<T>>::insert(project_id, now.clone());
        // 	// check what if called again, its done with `ensure_staking_period_set_once_project_id`
        // 	T::SchellingGameSharedSource::set_to_staking_period_pe_link(key.clone(), now.clone())?;
        // 	T::SchellingGameSharedSource::create_tree_helper_link(key, 3)?;

        // 	Self::deposit_event(Event::StakinPeriodStarted { project_id, block_number: now });

        // 	Ok(())
        // }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn apply_jurors(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let block_number =
                Self::get_block_number_of_schelling_game(department_required_fund_id)?;

            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: block_number.clone(),
            };

            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::apply_jurors_helper_link(
                key,
                phase_data,
                who.clone(),
                stake,
            )?;
            Self::deposit_event(Event::ApplyJurors {
                department_required_fund_id,
                block_number,
                account: who,
            });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn pass_period(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let block_number =
                Self::get_block_number_of_schelling_game(department_required_fund_id)?;

            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: block_number.clone(),
            };

            let now = <frame_system::Pallet<T>>::block_number();
            let phase_data = Self::get_phase_data();
            T::SchellingGameSharedSource::change_period_link(key, phase_data, now)?;
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn draw_jurors(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
            iterations: u64,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let block_number =
                Self::get_block_number_of_schelling_game(department_required_fund_id)?;

            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: block_number.clone(),
            };

            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::draw_jurors_helper_link(key, phase_data, iterations)?;

            Ok(())
        }

        // Unstaking
        // Stop drawn juror to unstake ✔️
        #[pallet::call_index(5)]
        #[pallet::weight(0)]
        pub fn unstaking(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number =
                Self::get_block_number_of_schelling_game(department_required_fund_id)?;
            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: block_number.clone(),
            };

            T::SchellingGameSharedSource::unstaking_helper_link(key, who)?;
            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(0)]
        pub fn commit_vote(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
            vote_commit: [u8; 32],
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number =
                Self::get_block_number_of_schelling_game(department_required_fund_id)?;
            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: block_number.clone(),
            };

            T::SchellingGameSharedSource::commit_vote_helper_link(key, who, vote_commit)?;
            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(0)]
        pub fn reveal_vote(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
            choice: u128,
            salt: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let block_number =
                Self::get_block_number_of_schelling_game(department_required_fund_id)?;
            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: block_number.clone(),
            };

            T::SchellingGameSharedSource::reveal_vote_two_choice_helper_link(
                key, who, choice, salt,
            )?;
            Ok(())
        }

        // #[pallet::call_index(8)]
        // #[pallet::weight(0)]
        // pub fn get_incentives(
        //     origin: OriginFor<T>,
        //     department_required_fund_id: DepartmentRequiredFundId,
        // ) -> DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     let block_number =
        //         Self::get_block_number_of_schelling_game(department_required_fund_id)?;
        //     // let key = SumTreeName::DepartmentRequiredFund {
        //     //     department_required_fund_id,
        //     //     block_number: block_number.clone(),
        //     // };

        //     let phase_data = Self::get_phase_data();
        //     // T::SchellingGameSharedSource::get_incentives_two_choice_helper_link(
        //     //     key, phase_data, who,
        //     // )?;
        //     Ok(())
        // }

        #[pallet::call_index(9)]
        #[pallet::weight(0)]
        pub fn add_incentive_count(
            origin: OriginFor<T>,
            department_required_fund_id: DepartmentRequiredFundId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number =
                Self::get_block_number_of_schelling_game(department_required_fund_id)?;
            let key = SumTreeName::DepartmentRequiredFund {
                department_required_fund_id,
                block_number: block_number.clone(),
            };

            let (juror_game_result, stake) =
                T::SchellingGameSharedSource::get_result_of_juror(key.clone(), who.clone())?;

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


        #[pallet::call_index(10)]
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
    }
}
