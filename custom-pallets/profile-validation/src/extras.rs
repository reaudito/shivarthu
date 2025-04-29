use crate::*;

impl<T: Config> CitizenDetailsPost<T> {
    pub fn new(
        citizen_id: CitizenId,
        created_by: T::AccountId,
        content: Content,
        location: LocationDetails,
    ) -> Self {
        CitizenDetailsPost {
            created: new_who_and_when::<T>(created_by.clone()),
            content,
            location,
            citizen_id,
            owner: created_by,
            edited: false,
            hidden: false,
            upvotes_count: 0,
            downvotes_count: 0,
        }
    }

    pub fn ensure_owner(&self, account: &T::AccountId) -> DispatchResult {
        ensure!(self.is_owner(account), Error::<T>::NotAPostOwner);
        Ok(())
    }

    pub fn is_owner(&self, account: &T::AccountId) -> bool {
        self.owner == *account
    }
}

impl<T: Config> ChallengeEvidencePost<T> {
    pub fn new(
        kyc_profile_id: T::AccountId,
        created_by: T::AccountId,
        content: Content,
        post_id_if_comment: Option<ChallengePostId>,
    ) -> Self {
        ChallengeEvidencePost {
            created: new_who_and_when::<T>(created_by.clone()),
            owner: created_by,
            kyc_profile_id,
            content,
            post_id_if_comment,
            is_comment: false,
        }
    }

    pub fn ensure_owner(&self, account: &T::AccountId) -> DispatchResult {
        ensure!(self.is_owner(account), Error::<T>::NotAPostOwner);
        Ok(())
    }

    pub fn is_owner(&self, account: &T::AccountId) -> bool {
        self.owner == *account
    }
}

impl<T: Config> Pallet<T> {
    pub(super) fn get_phase_data() -> PhaseData<T> {
        let evidence_length: u64 = T::EvidenceLength::get();
        let end_of_staking_time: u64 = T::EndOfStakingTime::get();
        let staking_length: u64 = T::StakingLength::get();
        let drawing_length: u64 = T::DrawingLength::get();
        let commit_length: u64 = T::CommitLength::get();
        let vote_length: u64 = T::VoteLength::get();
        let appeal_length: u64 = T::AppealLength::get();
        let max_draws: u64 = T::MaxDraws::get();
        let min_number_juror_staked: u64 = T::MinNumberJurorStaked::get();
        let min_juror_stake: u64 = T::MinJurorStake::get();
        let juror_incentives: (u64, u64) = T::JurorIncentives::get();

        T::SchellingGameSharedSource::create_phase_with_all_data(
            evidence_length,
            end_of_staking_time,
            staking_length,
            drawing_length,
            commit_length,
            vote_length,
            appeal_length,
            max_draws,
            min_number_juror_staked,
            min_juror_stake,
            juror_incentives,
        )
    }

    // pub(super) fn get_citizen_accountid(
    // 	citizenid: CitizenId,
    // ) -> Result<T::AccountId, DispatchError> {
    // 	let profile = Self::citizen_profile(citizenid).ok_or(Error::<T>::CitizenDoNotExists)?;
    // 	Ok(profile.owner)
    // }

    pub(super) fn _fund_profile_account() -> T::AccountId {
        PALLET_ID.into_sub_account_truncating(1)
    }

    pub(super) fn _u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
        input.saturated_into::<BalanceOf<T>>()
    }

    pub(super) fn balance_to_u64_saturated(input: BalanceOf<T>) -> u64 {
        input.saturated_into::<u64>()
    }

    pub(super) fn _u64_to_block_saturated(input: u64) -> BlockNumberOf<T> {
        input.saturated_into::<BlockNumberOf<T>>()
    }

    pub fn get_challengers_evidence(
        profile_user_account: T::AccountId,
        offset: u64,
        limit: u16,
    ) -> Vec<ChallengePostId> {
        let mut data = <ChallengerEvidenceId<T>>::iter_prefix_values(&profile_user_account)
            .skip(offset as usize)
            .take(limit as usize)
            .collect::<Vec<_>>();
        data.sort();
        data.reverse();
        data
    }

    pub fn get_evidence_period_end_block(profile_user_account: T::AccountId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();
        let block_number = <ValidationBlock<T>>::get(&profile_user_account);

        let key = SumTreeName::ProfileValidation {
            citizen_address: profile_user_account.clone(),
            block_number,
        };

        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_evidence_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn get_staking_period_end_block(profile_user_account: T::AccountId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();
        let block_number = <ValidationBlock<T>>::get(&profile_user_account);

        let key = SumTreeName::ProfileValidation {
            citizen_address: profile_user_account.clone(),
            block_number,
        };

        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_staking_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn get_drawing_period_end(profile_user_account: T::AccountId) -> (u64, u64, bool) {
        let block_number = <ValidationBlock<T>>::get(&profile_user_account);

        let key = SumTreeName::ProfileValidation {
            citizen_address: profile_user_account.clone(),
            block_number,
        };
        let phase_data = Self::get_phase_data();

        let result =
            T::SchellingGameSharedSource::get_drawing_period_end_helper_link(key, phase_data);
        result
    }

    pub fn get_commit_period_end_block(profile_user_account: T::AccountId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();
        let block_number = <ValidationBlock<T>>::get(&profile_user_account);

        let key = SumTreeName::ProfileValidation {
            citizen_address: profile_user_account.clone(),
            block_number,
        };
        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_commit_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn get_vote_period_end_block(profile_user_account: T::AccountId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();
        let block_number = <ValidationBlock<T>>::get(&profile_user_account);

        let key = SumTreeName::ProfileValidation {
            citizen_address: profile_user_account.clone(),
            block_number,
        };
        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_vote_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn selected_as_juror(profile_user_account: T::AccountId, who: T::AccountId) -> bool {
        let block_number = <ValidationBlock<T>>::get(&profile_user_account);

        let key = SumTreeName::ProfileValidation {
            citizen_address: profile_user_account.clone(),
            block_number,
        };

        let result = T::SchellingGameSharedSource::selected_as_juror_helper_link(key, who);
        result
    }

    pub fn profile_fund_required(profile_user_account: T::AccountId) -> Option<u64> {
        let registration_fee = Self::profile_registration_challenge_fees();
        let total_funded = Self::total_fund_for_profile_collected(profile_user_account);
        let registration_fee_u64 = Self::balance_to_u64_saturated(registration_fee);
        let total_fund_u64 = Self::balance_to_u64_saturated(total_funded);
        let fund_required = registration_fee_u64.checked_sub(total_fund_u64);
        fund_required
    }
}
