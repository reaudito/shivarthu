#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
///
/// To Do:
/// Add profile ✅
/// Crowdfund for profile stake ✅
/// Add another account in case you loose account access
/// Appeal in case of fradulent account
/// Clean the storage after are incentives are given
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
pub mod weights;
pub use weights::*;

mod extras;
mod permissions;
mod types;

use crate::types::{ChallengeEvidencePost, ChallengerFundInfo, LocationDetails, ProfileFundInfo};
use frame_support::sp_runtime::traits::AccountIdConversion;
use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
use frame_support::{
    traits::{Currency, ExistenceRequirement, OnUnbalanced, ReservableCurrency, WithdrawReasons},
    PalletId,
};
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

use pallet_schelling_game_shared::types::{
    Period, PhaseData, RangePoint, SchellingGameType, WinningDecision,
};
use pallet_sortition_sum_game::types::SumTreeName;
use pallet_support::{new_who_and_when, Content, WhoAndWhenOf};
use trait_schelling_game_shared::SchellingGameSharedLink;
use trait_shared_storage::SharedStorageLink;
pub use types::{CitizenDetailsPost, FIRST_CHALLENGE_POST_ID, FIRST_CITIZEN_ID};
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type ProfileFundInfoOf<T> = ProfileFundInfo<BalanceOf<T>, AccountIdOf<T>>;
type ChallengerFundInfoOf<T> = ChallengerFundInfo<BalanceOf<T>, BlockNumberFor<T>, AccountIdOf<T>>;
pub type BlockNumberOf<T> = BlockNumberFor<T>;
type CitizenId = u64;
type ChallengePostId = u64;
type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

const PALLET_ID: PalletId = PalletId(*b"ex/cfund");

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

        type SchellingGameSharedSource: SchellingGameSharedLink<
            SumTreeName = SumTreeName<Self::AccountId, BlockNumberOf<Self>>,
            SchellingGameType = SchellingGameType,
            BlockNumber = BlockNumberOf<Self>,
            AccountId = AccountIdOf<Self>,
            Balance = BalanceOf<Self>,
            RangePoint = RangePoint,
            Period = Period,
            WinningDecision = WinningDecision,
            PhaseData = PhaseData<Self>,
        >;

        type SharedStorageSource: SharedStorageLink<AccountId = AccountIdOf<Self>>;

        type Currency: ReservableCurrency<Self::AccountId>;
        /// Handler for the unbalanced increment when rewarding (minting rewards)
        type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

        /// Handler for the unbalanced decrement when slashing (burning collateral)
        type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;

        #[pallet::constant]
        type EvidenceLength: Get<u64>;

        #[pallet::constant]
        type EndOfStakingTime: Get<u64>;

        #[pallet::constant]
        type StakingLength: Get<u64>;

        #[pallet::constant]
        type DrawingLength: Get<u64>;

        #[pallet::constant]
        type CommitLength: Get<u64>;

        #[pallet::constant]
        type VoteLength: Get<u64>;

        #[pallet::constant]
        type AppealLength: Get<u64>;

        #[pallet::constant]
        type MaxDraws: Get<u64>;

        #[pallet::constant]
        type MinNumberJurorStaked: Get<u64>;

        #[pallet::constant]
        type MinJurorStake: Get<u64>;

        #[pallet::constant]
        type JurorIncentives: Get<(u64, u64)>;
    }

    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::type_value]
    pub fn DefaultForNextCitizenId() -> CitizenId {
        FIRST_CITIZEN_ID
    }

    #[pallet::storage]
    #[pallet::getter(fn next_citizen_id)]
    pub type NextCitizenId<T: Config> =
        StorageValue<_, CitizenId, ValueQuery, DefaultForNextCitizenId>;

    #[pallet::storage]
    #[pallet::getter(fn get_citizen_id)]
    pub type GetCitizenId<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, CitizenId>;

    #[pallet::storage]
    #[pallet::getter(fn citizen_profile)]
    pub type CitizenProfile<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, CitizenDetailsPost<T>>; // Peer account id => Peer Profile Hash

    // Registration Fees

    #[pallet::type_value]
    pub fn DefaultRegistrationFee<T: Config>() -> BalanceOf<T> {
        1000u128.saturated_into::<BalanceOf<T>>()
    }
    // Registration challenge fees
    #[pallet::type_value]
    pub fn DefaultRegistrationChallengeFee<T: Config>() -> BalanceOf<T> {
        100u128.saturated_into::<BalanceOf<T>>()
    }

    #[pallet::storage]
    #[pallet::getter(fn profile_registration_fees)]
    pub type RegistrationFee<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, DefaultRegistrationFee<T>>;

    #[pallet::storage]
    #[pallet::getter(fn profile_registration_challenge_fees)]
    pub type RegistrationChallengeFee<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, DefaultRegistrationChallengeFee<T>>;

    #[pallet::storage]
    #[pallet::getter(fn profile_fund_details)]
    pub type ProfileFundDetails<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        ProfileFundInfoOf<T>,
    >; // Profile account id and (funder accountid, profile fund info)

    #[pallet::storage]
    #[pallet::getter(fn total_fund_for_profile_collected)]
    pub type ProfileTotalFundCollected<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validation_block)]
    pub type ValidationBlock<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BlockNumberOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn challenger_fund)]
    pub type ChallengerFundDetails<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, ChallengerFundInfoOf<T>>; // Profile account id and challenger fund info

    /// There is a single challenger, but they can have multiple posts
    #[pallet::storage]
    #[pallet::getter(fn challenger_evidence_query)]
    pub type ChallengerEvidenceId<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        ChallengePostId,
    >; // profile accountid, challenger accountid => Challenge post id

    #[pallet::type_value]
    pub fn DefaultForNextChallengePostId() -> ChallengePostId {
        FIRST_CHALLENGE_POST_ID
    }

    #[pallet::storage]
    #[pallet::getter(fn next_challenge_post_count)]
    pub type NextChallengePostId<T: Config> =
        StorageValue<_, ChallengePostId, ValueQuery, DefaultForNextChallengePostId>;

    #[pallet::storage]
    #[pallet::getter(fn challenge_post_comment)]
    pub type ChallengePostCommentIds<T: Config> =
        StorageMap<_, Blake2_128Concat, ChallengePostId, Vec<ChallengePostId>, ValueQuery>; // challenge post id => Vec<Comment Post It>

    #[pallet::storage]
    #[pallet::getter(fn challenge_post)]
    pub type ChallengePost<T: Config> =
        StorageMap<_, Blake2_128Concat, ChallengePostId, ChallengeEvidencePost<T>>; // challenge post id => post

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
        CreateCitizen(T::AccountId, CitizenId),
        ProfileFund {
            profile: T::AccountId,
            funder: T::AccountId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        NoMoreUpdates,
        CitizenDoNotExists,
        ProfileFundExists,
        PostAlreadyExists,
        ProfileIsAlreadyValidated,
        ChallengeExits,
        ChallengeDoesNotExists,
        CommentExists,
        IsComment,
        ProfileFundNotExists,
        ChallengerFundInfoExists,
        NotProfileUser,
        NotEvidencePeriod,
        CitizenNotApproved,
        NotAPostOwner,
        AmountFundedGreaterThanRequired,
        ProfileFundAlreadyReturned,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add citizen
        /// <pre>
        /// Get the count from NextCitizenId
        /// If CitizenId exists update the content, only if `ProfileTotalFundCollected` is zero
        /// If CitizenId doesn't exists insert the content, and increment the `NextCitizenId`
        /// </pre>

        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn add_citizen(
            origin: OriginFor<T>,
            content: Content,
            location: LocationDetails,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let count = Self::next_citizen_id();
            match <GetCitizenId<T>>::get(&who) {
                Some(citizen_id) => {
                    let total_funded = <ProfileTotalFundCollected<T>>::get(who.clone());
                    if total_funded == 0u128.saturated_into::<BalanceOf<T>>() {
                        let new_post: CitizenDetailsPost<T> = CitizenDetailsPost::new(
                            citizen_id,
                            who.clone(),
                            content.clone(),
                            location.clone(),
                        );
                        <CitizenProfile<T>>::insert(who.clone(), new_post);
                        Ok(())
                    } else {
                        Err(Error::<T>::NoMoreUpdates)?
                    }
                }
                None => {
                    <GetCitizenId<T>>::insert(&who, count);

                    let new_post: CitizenDetailsPost<T> = CitizenDetailsPost::new(
                        count,
                        who.clone(),
                        content.clone(),
                        location.clone(),
                    );

                    <CitizenProfile<T>>::insert(who.clone(), new_post);
                    NextCitizenId::<T>::mutate(|n| {
                        *n += 1;
                    });
                    Self::deposit_event(Event::CreateCitizen(who, count));
                    Ok(())
                }
            }
        }

        /// # Crowdfunding of Profile
        ///
        /// Allows users to contribute funds to a profile and manages the associated data for crowdfunding.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `profile_user_account`: The account ID of the profile to fund.
        /// - `amount_to_fund`: The amount of funds to be added to the profile's crowdfunding.
        ///
        /// ## Errors
        ///
        /// This function can return an error if the amount to fund is greater than the required fund.
        ///
        /// ## Storage
        ///
        /// - `ValidationBlock`:  Stores the block number to be used for the profile's validation when `amount_to_fund` equals `required_fund`.
        /// - `ProfileFundDetails`: Stores details of funds deposited by users for a specific profile.
        /// - `ProfileTotalFundCollected`: Keeps track of the total funds collected for each profile.
        /// - `RegistrationFee`: Retrieves the registration fee required for profile validation.
        /// - `GetCitizenId`: Storage map that associates a citizen's account address with their Citizen ID.
        ///
        /// ## Usage
        ///
        /// Call this function to contribute funds to a profile and update the associated storage items.
        /// Checks are performed to ensure the profile exists and that the funded amount is not greater
        /// than required. If the funded amount matches the required amount, the profile validation is marked
        /// as completed, and a link is set to the evidence period in the Schelling Game.
        ///
        /// ```rust,ignore
        /// #[pallet::call]
        /// fn add_profile_stake(
        ///     origin: OriginFor<T>,
        ///     profile_user_account: T::AccountId,
        ///     amount_to_fund: BalanceOf<T>,
        /// ) -> DispatchResult {
        ///     // implementation
        /// }
        /// ```

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn add_profile_stake(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
            amount_to_fund: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure that the `profile_user_account`` exists in `GetCitizenId` storage.
            Self::ensure_account_id_has_profile(profile_user_account.clone())?;

            // Retrieve the registration fee required for profile validation.
            let registration_fee = <RegistrationFee<T>>::get();

            // Get the total funds already collected for the profile.
            let total_funded = <ProfileTotalFundCollected<T>>::get(profile_user_account.clone());

            // Calculate the required fund by subtracting the total funded from the registration fee.
            let required_fund = registration_fee
                .checked_sub(&total_funded)
                .expect("Overflow");

            // Check if the amount_to_fund is less than or equal to the required fund.
            if amount_to_fund <= required_fund {
                if amount_to_fund == required_fund {
                    // If the funded amount matches the required amount, update variables required for profile validation.
                    let now = <frame_system::Pallet<T>>::block_number();
                    let key = SumTreeName::ProfileValidation {
                        citizen_address: profile_user_account.clone(),
                        block_number: now.clone(),
                    };
                    <ValidationBlock<T>>::insert(&profile_user_account, now);

                    // Set a link to the evidence period in the Schelling Game.
                    T::SchellingGameSharedSource::set_to_evidence_period_link(key, now)?;
                }

                // Withdraw funds from the funder's account.
                let _ = <T as pallet::Config>::Currency::withdraw(
                    &who,
                    amount_to_fund.clone(),
                    WithdrawReasons::TRANSFER,
                    ExistenceRequirement::AllowDeath,
                )?;

                // Update the profile fund details for the funder.
                match <ProfileFundDetails<T>>::get(profile_user_account.clone(), who.clone()) {
                    Some(mut profile_fund_info) => {
                        let deposit = profile_fund_info.deposit;
                        let new_deposit = deposit.checked_add(&amount_to_fund).expect("Overflow");
                        profile_fund_info.deposit = new_deposit;
                        <ProfileFundDetails<T>>::insert(
                            profile_user_account.clone(),
                            who.clone(),
                            profile_fund_info,
                        );
                    }
                    None => {
                        let profile_fund_info = ProfileFundInfo {
                            funder_account_id: who.clone(),
                            validation_account_id: profile_user_account.clone(),
                            deposit: amount_to_fund.clone(),
                            deposit_returned: false,
                        };
                        <ProfileFundDetails<T>>::insert(
                            profile_user_account.clone(),
                            who.clone(),
                            profile_fund_info,
                        );
                    }
                }

                // Update the total funds collected for the profile.
                let next_total_fund = total_funded.checked_add(&amount_to_fund).expect("overflow");
                <ProfileTotalFundCollected<T>>::insert(
                    profile_user_account.clone(),
                    next_total_fund,
                );

                // Emit a ProfileFund event.
                Self::deposit_event(Event::ProfileFund {
                    profile: profile_user_account,
                    funder: who,
                });
            } else {
                // Return an error if the funded amount is greater than required.
                Err(Error::<T>::AmountFundedGreaterThanRequired)?
            }

            Ok(())
        }

        // Add fees for challenge profile ✔️
        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn challenge_profile(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
            content: Content,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::ensure_account_id_has_profile(profile_user_account.clone())?;
            let now = <frame_system::Pallet<T>>::block_number();

            let fees = Self::profile_registration_challenge_fees();

            let challenger_fund_info = ChallengerFundInfo {
                challengerid: who.clone(),
                deposit: fees,
                start: now.clone(),
                challenge_completed: false,
            };

            let challenger_fund_details = <ChallengerFundDetails<T>>::get(&profile_user_account);
            match challenger_fund_details {
                Some(_value) => Err(Error::<T>::ChallengeExits)?,
                None => {
                    let _ = <T as pallet::Config>::Currency::withdraw(
                        &who,
                        fees.clone(),
                        WithdrawReasons::TRANSFER,
                        ExistenceRequirement::AllowDeath,
                    )?;
                    <ChallengerFundDetails<T>>::insert(&profile_user_account, challenger_fund_info);
                }
            }

            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };

            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::set_to_staking_period_link(key.clone(), phase_data, now)?;
            T::SchellingGameSharedSource::create_tree_helper_link(key.clone(), 3)?;

            let count = Self::next_challenge_post_count();

            let challenge_evidence_post: ChallengeEvidencePost<T> = ChallengeEvidencePost::new(
                profile_user_account.clone(),
                who.clone(),
                content,
                None,
            );

            match <ChallengerEvidenceId<T>>::get(&profile_user_account, &who) {
                None => {
                    <ChallengePost<T>>::insert(&count, challenge_evidence_post);
                    NextChallengePostId::<T>::mutate(|n| {
                        *n += 1;
                    });

                    <ChallengerEvidenceId<T>>::insert(&profile_user_account, &who, count);
                }
                Some(_hash) => Err(Error::<T>::PostAlreadyExists)?,
            }
            Ok(())
        }

        // #[pallet::call_index(2)]
        // #[pallet::weight(Weight::from_parts(10_000, u64::MAX) + T::DbWeight::get().reads_writes(2,2))]
        // pub fn challenge_evidence(
        // 	origin: OriginFor<T>,
        // 	profile_citizenid: CitizenId,
        // 	content: Content,
        // ) -> DispatchResult {
        // 	let who = ensure_signed(origin)?;
        // 	let citizen_account_id = Self::get_citizen_accountid(profile_citizenid)?;
        // 	let count = Self::next_challenge_post_count();
        // 	let challenge_evidence_post =
        // 		ChallengeEvidencePost::new(citizen_account_id, who.clone(), content, None);
        // 	match <ChallengerEvidenceId<T>>::get(&profile_citizenid, &who) {
        // 		None => {
        // 			<ChallengePost<T>>::insert(&count, challenge_evidence_post);
        // 			NextChallengePostId::<T>::mutate(|n| {
        // 				*n += 1;
        // 			});

        // 			<ChallengerEvidenceId<T>>::insert(&profile_citizenid, &who, count);
        // 		},
        // 		Some(_hash) => {
        // 			Err(Error::<T>::PostAlreadyExists)?
        // 			// match <ChallengerFundDetails<T>>::get(&profile_citizenid) {
        // 			// 	Some(_challengerfundinfo) => {
        // 			// 		Err(Error::<T>::ChallengerFundAddedCanNotUpdate)?
        // 			// 	},
        // 			// 	None => {
        // 			// 		// Update challenger profile
        // 			// 		<ChallengePost<T>>::insert(&count, challenge_evidence_post);
        // 			// 		let newcount =
        // 			// 			count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
        // 			// 		<ChallengePostCount<T>>::put(newcount);
        // 			// 		<ChallengerEvidenceId<T>>::insert(&profile_citizenid, &who, count);
        // 			// 	},
        // 			// }
        // 		},
        // 	}
        // 	Ok(())
        // }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn challenge_comment_create(
            origin: OriginFor<T>,
            post_id: ChallengePostId,
            content: Content,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let count = Self::next_challenge_post_count();
            let main_evidence_post = Self::challenge_post(post_id).unwrap();
            let challenge_evidence_post = ChallengeEvidencePost::new(
                main_evidence_post.kyc_profile_id,
                who,
                content,
                Some(post_id),
            );

            match <ChallengePost<T>>::get(&post_id) {
                None => Err(Error::<T>::ChallengeDoesNotExists)?,
                Some(challenge_evidence_post_c) => {
                    if challenge_evidence_post_c.is_comment == false {
                        <ChallengePost<T>>::insert(&count, challenge_evidence_post);
                        NextChallengePostId::<T>::mutate(|n| {
                            *n += 1;
                        });
                        let mut comment_ids = <ChallengePostCommentIds<T>>::get(&post_id);
                        match comment_ids.binary_search(&count) {
                            Ok(_) => Err(Error::<T>::CommentExists)?,
                            Err(index) => {
                                comment_ids.insert(index, count.clone());
                                <ChallengePostCommentIds<T>>::insert(&post_id, &comment_ids);
                            }
                        }
                    } else {
                        Err(Error::<T>::IsComment)?
                    }
                }
            }

            Ok(())
        }

        // // Does citizen exists ✔️
        // // Has the citizen added profile fund ✔️
        // // Create tree ✔️
        // // Check evidence has been submitted
        // #[pallet::call_index(4)]
        // #[pallet::weight(Weight::from_parts(10_000, u64::MAX) + T::DbWeight::get().reads_writes(2,2))]
        // pub fn challenge_profile(
        // 	origin: OriginFor<T>,
        // 	profile_citizenid: CitizenId,
        // ) -> DispatchResult {
        // 	let who = ensure_signed(origin)?;
        // 	let key = SumTreeName::UniqueIdenfier1 {
        // 		citizen_id: profile_citizenid,
        // 		name: "challengeprofile".as_bytes().to_vec(),
        // 	};
        // 	let phase_data = Self::get_phase_data();
        // 	let now = <frame_system::Pallet<T>>::block_number();
        // 	let _citizen_account_id = Self::get_citizen_accountid(profile_citizenid)?;
        // 	match <ProfileFundDetails<T>>::get(&profile_citizenid) {
        // 		Some(profilefundinfo) => {
        // 			if profilefundinfo.validated == true {
        // 				Err(Error::<T>::ProfileIsAlreadyValidated)?;
        // 			} else {
        // 				let _evidence_stake_block_number = profilefundinfo.start; // remove the profile fund info start

        // 				T::SchellingGameSharedSource::set_to_staking_period_link(
        // 					key.clone(),
        // 					phase_data,
        // 					now,
        // 				)?;
        // 			}
        // 		},
        // 		None => {
        // 			Err(Error::<T>::ProfileFundNotExists)?;
        // 		},
        // 	}
        // 	let deposit = <RegistrationChallengeFee<T>>::get();
        // 	let imb = <T as pallet::Config>::Currency::withdraw(
        // 		&who,
        // 		deposit,
        // 		WithdrawReasons::TRANSFER,
        // 		ExistenceRequirement::AllowDeath,
        // 	)?;

        // 	<T as pallet::Config>::Currency::resolve_creating(&Self::fund_profile_account(), imb);

        // 	match <ChallengerFundDetails<T>>::get(&profile_citizenid) {
        // 		// 📝 To write update stake for reapply
        // 		Some(_challengerfundinfo) => Err(Error::<T>::ChallengerFundInfoExists)?,
        // 		None => {
        // 			let challenger_fund_info = ChallengerFundInfo {
        // 				challengerid: who,
        // 				deposit,
        // 				start: now,
        // 				challenge_completed: false,
        // 			};
        // 			<ChallengerFundDetails<T>>::insert(&profile_citizenid, challenger_fund_info);
        // 		},
        // 	}
        //      T::SchellingGameSharedSource::create_tree_helper_link(key, 3)?;

        // 	 Ok(())
        // }

        // May be you need to check challeger fund details exists
        #[pallet::call_index(5)]
        #[pallet::weight(0)]
        pub fn pass_period(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };

            let now = <frame_system::Pallet<T>>::block_number();
            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::change_period_link(key, phase_data, now)?;

            Ok(())
        }

        // To Do
        // Apply jurors or stake ✔️
        // Update stake
        // Draw jurors ✔️
        // Unstaking non selected jurors ✔️
        // Commit vote ✔️
        // Reveal vote ✔️
        // Get winning decision ✔️
        // Incentive distribution ✔️

        // Staking
        // 1. Check for minimum stake ✔️
        // 2. Check period is Staking ✔️
        // 3. Number of people staked

        #[pallet::call_index(6)]
        #[pallet::weight(0)]
        pub fn apply_jurors(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };

            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::apply_jurors_helper_link(key, phase_data, who, stake)?;

            Ok(())
        }

        // Draw jurors
        // Check period is drawing ✔️
        // Check mininum number of juror staked ✔️
        // Improvements
        // Set stake to zero so that they are not drawn again
        // Store the drawn juror stake in hashmap storage
        // Add min draws along with max draws
        #[pallet::call_index(7)]
        #[pallet::weight(0)]
        pub fn draw_jurors(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
            iterations: u64,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };
            let phase_data = Self::get_phase_data();

            T::SchellingGameSharedSource::draw_jurors_helper_link(key, phase_data, iterations)?;

            Ok(())
        }

        // Unstaking
        // Stop drawn juror to unstake ✔️
        #[pallet::call_index(8)]
        #[pallet::weight(0)]
        pub fn unstaking(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };
            T::SchellingGameSharedSource::unstaking_helper_link(key, who)?;
            Ok(())
        }

        #[pallet::call_index(9)]
        #[pallet::weight(0)]
        pub fn commit_vote(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
            vote_commit: [u8; 32],
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };
            T::SchellingGameSharedSource::commit_vote_helper_link(key, who, vote_commit)?;
            Ok(())
        }

        #[pallet::call_index(10)]
        #[pallet::weight(0)]
        pub fn reveal_vote(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
            choice: u128,
            salt: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };

            T::SchellingGameSharedSource::reveal_vote_two_choice_helper_link(
                key, who, choice, salt,
            )?;

            Ok(())
        }

        #[pallet::call_index(11)]
        #[pallet::weight(0)]
        pub fn get_incentives(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(&profile_user_account);

            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };
            let phase_data = Self::get_phase_data();
            T::SchellingGameSharedSource::get_incentives_two_choice_helper_link(
                key, phase_data, who,
            )?;
            Ok(())
        }

        #[pallet::call_index(12)]
        #[pallet::weight(0)]
        pub fn return_profile_stake(
            origin: OriginFor<T>,
            profile_user_account: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(&profile_user_account);
            let key = SumTreeName::ProfileValidation {
                citizen_address: profile_user_account.clone(),
                block_number,
            };
            let now = <frame_system::Pallet<T>>::block_number();
            let phase_data = Self::get_phase_data();

            let period = T::SchellingGameSharedSource::get_period_link(key.clone()).unwrap();
            if period == Period::Execution {
                let decision: WinningDecision =
                    T::SchellingGameSharedSource::get_winning_decision_value(key.clone())?;
                if decision == WinningDecision::WinnerNo {
                    match <ProfileFundDetails<T>>::get(profile_user_account.clone(), who.clone()) {
                        Some(mut profile_fund_info) => {
                            if profile_fund_info.deposit_returned == false {
                                let r = <T as pallet::Config>::Currency::deposit_into_existing(
                                    &who,
                                    profile_fund_info.deposit,
                                )
                                .ok()
                                .unwrap();
                                <T as pallet::Config>::Reward::on_unbalanced(r);
                                profile_fund_info.deposit_returned = true;
                                <ProfileFundDetails<T>>::insert(
                                    profile_user_account.clone(),
                                    who.clone(),
                                    profile_fund_info,
                                );
                            } else {
                                Err(Error::<T>::ProfileFundAlreadyReturned)?;
                            }
                        }
                        None => {
                            Err(Error::<T>::ProfileFundNotExists)?;
                        }
                    }
                }
            } else if period == Period::Evidence {
                T::SchellingGameSharedSource::ensure_time_for_staking_over_link(
                    key, phase_data, now,
                )?;
                match <ProfileFundDetails<T>>::get(profile_user_account.clone(), who.clone()) {
                    Some(mut profile_fund_info) => {
                        if profile_fund_info.deposit_returned == false {
                            let r = <T as pallet::Config>::Currency::deposit_into_existing(
                                &who,
                                profile_fund_info.deposit,
                            )
                            .ok()
                            .unwrap();
                            <T as pallet::Config>::Reward::on_unbalanced(r);
                            profile_fund_info.deposit_returned = true;
                            <ProfileFundDetails<T>>::insert(
                                profile_user_account.clone(),
                                who.clone(),
                                profile_fund_info,
                            );
                        } else {
                            Err(Error::<T>::ProfileFundAlreadyReturned)?;
                        }
                    }
                    None => {
                        Err(Error::<T>::ProfileFundNotExists)?;
                    }
                }
            }

            Ok(())
        }

        #[pallet::call_index(13)]
        #[pallet::weight(0)]
        pub fn add_to_kyc_accounts(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let block_number = <ValidationBlock<T>>::get(&who);
            let key = SumTreeName::ProfileValidation {
                citizen_address: who.clone(),
                block_number,
            };
            let now = <frame_system::Pallet<T>>::block_number();
            let phase_data = Self::get_phase_data();

            let period = T::SchellingGameSharedSource::get_period_link(key.clone()).unwrap();

            if period == Period::Execution {
                let decision: WinningDecision =
                    T::SchellingGameSharedSource::get_winning_decision_value(key.clone())?;
                if decision == WinningDecision::WinnerNo {
                    T::SharedStorageSource::add_approved_citizen_address(who.clone())?;
                }
            } else if period == Period::Evidence {
                T::SchellingGameSharedSource::ensure_time_for_staking_over_link(
                    key, phase_data, now,
                )?;
                T::SharedStorageSource::add_approved_citizen_address(who.clone())?;
            }
            Ok(())
        }
    }
}
