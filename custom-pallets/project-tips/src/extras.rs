use crate::*;

impl<T: Config> Project<T> {
    pub fn new(
        project_id: ProjectId,
        department_id: DepartmentId,
        content: Content,
        tipping_name: TippingName,
        funding_needed: BalanceOf<T>,
        project_leader: T::AccountId,
    ) -> Self {
        Project {
            created: new_who_and_when::<T>(project_leader.clone()),
            project_id,
            department_id,
            content,
            tipping_name,
            funding_needed,
            project_leader,
            released: false,
        }
    }
}

impl<T: Config> Incentives<T> {
    pub fn new(number_of_games: u64, winner: u64, loser: u64, stake: u64) -> Self {
        Incentives {
            number_of_games: number_of_games,
            winner: winner,
            loser: loser,
            total_stake: stake,
            start: new_when_details::<T>(),
        }
    }
}

impl<T: Config> Pallet<T> {
    pub(super) fn get_phase_data() -> PhaseData<T> {
        T::SchellingGameSharedSource::create_phase_data(50, 5, 3, 100, (100, 100))
    }

    pub fn ensure_user_is_project_creator_and_project_exists(
        project_id: ProjectId,
        user: T::AccountId,
    ) -> DispatchResult {
        let project_option: Option<Project<T>> = <Projects<T>>::get(project_id);
        match project_option {
            Some(project) => {
                let project_leader = project.project_leader;
                ensure!(project_leader == user, Error::<T>::ProjectCreatorDontMatch);
            }
            None => Err(Error::<T>::ProjectDontExists)?,
        }

        Ok(())
    }

    pub fn ensure_staking_period_set_once_project_id(project_id: ProjectId) -> DispatchResult {
        let block_number_option = <ValidationBlock<T>>::get(project_id);
        match block_number_option {
            Some(_block) => Err(Error::<T>::ProjectIdStakingPeriodAlreadySet)?,
            None => Ok(()),
        }
    }

    pub fn get_block_number_of_schelling_game(
        project_id: ProjectId,
    ) -> Result<BlockNumberOf<T>, DispatchError> {
        let block_number_option = <ValidationBlock<T>>::get(project_id);
        let block_number = match block_number_option {
            Some(block_number) => block_number,
            None => Err(Error::<T>::BlockNumberProjectIdNotExists)?,
        };
        Ok(block_number)
    }

    pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
        input.saturated_into::<BalanceOf<T>>()
    }

    pub(super) fn u64_to_block_saturated(input: u64) -> BlockNumberOf<T> {
        input.saturated_into::<BlockNumberOf<T>>()
    }

    pub fn value_of_tipping_name(tipping: TippingName) -> TippingValue<BalanceOf<T>> {
        match tipping {
            TippingName::SmallTipper => TippingValue {
                max_tipping_value: 10_000u64.saturated_into::<BalanceOf<T>>(),
                stake_required: 10u64.saturated_into::<BalanceOf<T>>(),
            },
            TippingName::BigTipper => TippingValue {
                max_tipping_value: 100_000u64.saturated_into::<BalanceOf<T>>(),
                stake_required: 50u64.saturated_into::<BalanceOf<T>>(),
            },
            TippingName::SmallSpender => TippingValue {
                max_tipping_value: 1_000_000u64.saturated_into::<BalanceOf<T>>(),
                stake_required: 100u64.saturated_into::<BalanceOf<T>>(),
            },
            TippingName::MediumSpender => TippingValue {
                max_tipping_value: 10_000_000u64.saturated_into::<BalanceOf<T>>(),
                stake_required: 200u64.saturated_into::<BalanceOf<T>>(),
            },
            TippingName::BigSpender => TippingValue {
                max_tipping_value: 100_000_000u64.saturated_into::<BalanceOf<T>>(),
                stake_required: 500u64.saturated_into::<BalanceOf<T>>(),
            },
        }
    }

    // Block code start

    pub fn get_evidence_period_end_block(project_id: ProjectId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();

        let block_number = Self::get_block_number_of_schelling_game(project_id).unwrap();

        let key = SumTreeName::ProjectTips {
            project_id,
            block_number: block_number.clone(),
        };

        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_evidence_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn get_staking_period_end_block(project_id: ProjectId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();

        let block_number = Self::get_block_number_of_schelling_game(project_id).unwrap();

        let key = SumTreeName::ProjectTips {
            project_id,
            block_number: block_number.clone(),
        };

        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_staking_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn get_drawing_period_end(project_id: ProjectId) -> (u64, u64, bool) {
        let block_number = Self::get_block_number_of_schelling_game(project_id).unwrap();

        let key = SumTreeName::ProjectTips {
            project_id,
            block_number: block_number.clone(),
        };
        let phase_data = Self::get_phase_data();

        let result =
            T::SchellingGameSharedSource::get_drawing_period_end_helper_link(key, phase_data);
        result
    }

    pub fn get_commit_period_end_block(project_id: ProjectId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();

        let block_number = Self::get_block_number_of_schelling_game(project_id).unwrap();

        let key = SumTreeName::ProjectTips {
            project_id,
            block_number: block_number.clone(),
        };

        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_commit_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn get_vote_period_end_block(project_id: ProjectId) -> Option<u32> {
        let now = <frame_system::Pallet<T>>::block_number();

        let block_number = Self::get_block_number_of_schelling_game(project_id).unwrap();

        let key = SumTreeName::ProjectTips {
            project_id,
            block_number: block_number.clone(),
        };

        let phase_data = Self::get_phase_data();

        let result = T::SchellingGameSharedSource::get_vote_period_end_block_helper_link(
            key, phase_data, now,
        );
        result
    }

    pub fn selected_as_juror(project_id: ProjectId, who: T::AccountId) -> bool {
        let block_number = Self::get_block_number_of_schelling_game(project_id).unwrap();

        let key = SumTreeName::ProjectTips {
            project_id,
            block_number: block_number.clone(),
        };

        let result = T::SchellingGameSharedSource::selected_as_juror_helper_link(key, who);
        result
    }

    // Block code end
}
