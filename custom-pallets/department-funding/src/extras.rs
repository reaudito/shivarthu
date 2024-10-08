use crate::*;
use types::{TIME_FOR_STAKING_FUNDING_STATUS_FAILED, TIME_FOR_STAKING_FUNDING_STATUS_PASSED};

impl<T: Config> DepartmentRequiredFund<T> {
	pub fn new(
		department_required_fund_id: DepartmentRequiredFundId,
		department_id: DepartmentId,
		content: Content,
		tipping_name: TippingName,
		funding_needed: BalanceOf<T>,
		creator: T::AccountId,
	) -> Self {
		DepartmentRequiredFund {
			created: new_who_and_when::<T>(creator.clone()),
			department_required_fund_id,
			content,
			department_id,
			tipping_name,
			funding_needed,
			creator,
			released: false,
		}
	}
}

impl<T: Config> Pallet<T> {
	pub(super) fn get_phase_data() -> PhaseData<T> {
		T::SchellingGameSharedSource::create_phase_data(50, 5, 3, 100, (100, 100))
	}

	pub fn ensure_validation_to_do(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> DispatchResult {
		let bool_data = ValidateDepartmentRequiredFund::<T>::get(department_required_fund_id);
		ensure!(bool_data == true, Error::<T>::ValidationForDepartmentRequiredFundIdIsOff);

		Ok(())
	}

	pub fn get_department_id_from_department_required_fund_id(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Result<DepartmentId, DispatchError> {
		let department_required_fund_option =
			DepartmentRequiredFunds::<T>::get(department_required_fund_id);

		match department_required_fund_option {
			Some(department_required_fund) => Ok(department_required_fund.department_id),
			None => Err(Error::<T>::DepartmentRequiredFundDontExits)?,
		}
	}

	/// Ensures that a department can stake based on its current funding status and the time elapsed since the last status change.
	///
	/// # Parameters
	/// - `department_id`: The identifier of the department for which staking is being checked.
	///
	/// # Returns
	/// - `Ok(DepartmentFundingStatus)`: If the department can proceed with staking. The returned value contains the current block number and sets the status to `Processing`.
	/// - `Err(DispatchError)`: If the department cannot stake due to one of the following reasons:
	///   - **`FundingStatus::Processing`**: The department is already in the `Processing` state, meaning that a funding operation is currently ongoing.
	///   - **`FundingStatus::Failed`**: The department's last funding attempt failed, and the required waiting period (defined by `TIME_FOR_STAKING_FUNDING_STATUS_FAILED`) has not yet passed.
	///   - **`FundingStatus::Success`**: The department's last funding attempt succeeded, and the required waiting period (defined by `TIME_FOR_STAKING_FUNDING_STATUS_PASSED`) has not yet passed.
	///   - **`ConditionDontMatch`**: If the department's funding status doesn't match any known conditions.
	///
	/// # Logic
	/// 1. **Current Block Time**: The function first retrieves the current block number to timestamp the new funding status if applicable.
	/// 2. **Existing Status Check**: It checks if there is an existing funding status for the given department:
	///    - **Processing**: If the status is `Processing`, staking cannot proceed, and an error is returned.
	///    - **Failed**: If the status is `Failed`, it checks if the required waiting period has passed since the failure. If the time has passed, staking can proceed; otherwise, an error is returned.
	///    - **Success**: If the status is `Success`, it similarly checks if the required waiting period has passed. If it has, staking can proceed; otherwise, an error is returned.
	///    - **Other Conditions**: If the status does not match any of the above, an error is returned.
	/// 3. **No Existing Status**: If there is no existing status, staking is allowed, and the department's status is set to `Processing`.
	///
	/// # Errors
	/// - **`FundingStatusProcessing`**: Returned if the department is already in the `Processing` state.
	/// - **`ReapplicationTimeNotReached`**: Returned if the department is trying to reapply before the required waiting period has passed.
	/// - **`ConditionDontMatch`**: Returned if the department's funding status doesn't match any known conditions.
	pub fn ensure_can_stake_using_status(
		department_id: DepartmentId,
	) -> Result<DepartmentFundingStatus<BlockNumberOf<T>, FundingStatus>, DispatchError> {
		let now = <frame_system::Pallet<T>>::block_number();
		let department_funding_status =
			DepartmentFundingStatus { block_number: now, status: FundingStatus::Processing };
		let department_status_option =
			DepartmentFundingStatusForDepartmentId::<T>::get(department_id);
		match department_status_option {
			Some(department_status) => {
				let funding_status = department_status.status;
				if funding_status == FundingStatus::Processing {
					Err(Error::<T>::FundingStatusProcessing.into())
				} else if funding_status == FundingStatus::Failed {
					// else check 3 month if status faild or 6 months if status success to reapply for funding
					let status_failed_time = TIME_FOR_STAKING_FUNDING_STATUS_FAILED;
					let status_failed_time_block = Self::u64_to_block_saturated(status_failed_time);
					let funding_status_block = department_status.block_number;
					let time = now.checked_sub(&funding_status_block).expect("Overflow");
					if time >= status_failed_time_block {
						Ok(department_funding_status)
					} else {
						Err(Error::<T>::ReapplicationTimeNotReached.into())
					}
				} else if funding_status == FundingStatus::Success {
					let status_success_time = TIME_FOR_STAKING_FUNDING_STATUS_PASSED;
					let status_success_time_block =
						Self::u64_to_block_saturated(status_success_time);
					let funding_status_block = department_status.block_number;
					let time = now.checked_sub(&funding_status_block).expect("Overflow");
					if time >= status_success_time_block {
						Ok(department_funding_status)
					} else {
						Err(Error::<T>::ReapplicationTimeNotReached.into())
					}
				} else {
					Err(Error::<T>::ConditionDontMatch.into())
				}
			},
			None => Ok(department_funding_status),
		}
	}

	pub fn set_department_status(
		department_id: DepartmentId,
		funding_status: FundingStatus,
	) -> DispatchResult {
		match DepartmentFundingStatusForDepartmentId::<T>::get(department_id) {
			Some(mut department_status) => {
				department_status.status = funding_status;
				<DepartmentFundingStatusForDepartmentId<T>>::mutate(
					&department_id,
					|department_status_option| *department_status_option = Some(department_status),
				)
			},
			None => Err(Error::<T>::FundingStatusNotPresent)?,
		}

		Ok(())
	}

	// pub fn ensure_user_is_project_creator_and_project_exists(
	// 	project_id: ProjectId,
	// 	user: T::AccountId,
	// ) -> DispatchResult {
	// 	let project_option: Option<Project<T>> = Projects::get(project_id);
	// 	match project_option {
	// 		Some(project) => {
	// 			let creator = project.creator;
	// 			ensure!(creator == user, Error::<T>::ProjectCreatorDontMatch);
	// 		},
	// 		None => Err(Error::<T>::ProjectDontExists)?,
	// 	}

	// 	Ok(())
	// }

	// pub fn ensure_staking_period_set_once_project_id(project_id: ProjectId) -> DispatchResult {
	// 	let block_number_option = <ValidationProjectBlock<T>>::get(project_id);
	// 	match block_number_option {
	// 		Some(_block) => Err(Error::<T>::ProjectIdStakingPeriodAlreadySet)?,
	// 		None => Ok(()),
	// 	}
	// }

	pub fn get_block_number_of_schelling_game(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Result<BlockNumberOf<T>, DispatchError> {
		let block_number_option = <ValidationBlock<T>>::get(department_required_fund_id);
		let block_number = match block_number_option {
			Some(block_number) => block_number,
			None => Err(Error::<T>::BlockDepartmentRequiredFundIdNotExists)?,
		};
		Ok(block_number)
	}

	pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}

	pub(super) fn u64_to_block_saturated(input: u64) -> BlockNumberOf<T> {
		input.saturated_into::<BlockNumberOf<T>>()
	}

	pub(super) fn value_of_tipping_name(tipping: TippingName) -> TippingValue<BalanceOf<T>> {
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

	pub fn get_evidence_period_end_block(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let block_number =
			Self::get_block_number_of_schelling_game(department_required_fund_id).unwrap();

		let key = SumTreeName::DepartmentRequiredFund {
			department_required_fund_id,
			block_number: block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_evidence_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	// End block code start

	pub fn get_staking_period_end_block(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let block_number =
			Self::get_block_number_of_schelling_game(department_required_fund_id).unwrap();

		let key = SumTreeName::DepartmentRequiredFund {
			department_required_fund_id,
			block_number: block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_staking_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn get_drawing_period_end(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> (u64, u64, bool) {
		let block_number =
			Self::get_block_number_of_schelling_game(department_required_fund_id).unwrap();

		let key = SumTreeName::DepartmentRequiredFund {
			department_required_fund_id,
			block_number: block_number.clone(),
		};
		let phase_data = Self::get_phase_data();

		let result =
			T::SchellingGameSharedSource::get_drawing_period_end_helper_link(key, phase_data);
		result
	}

	pub fn get_commit_period_end_block(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let block_number =
			Self::get_block_number_of_schelling_game(department_required_fund_id).unwrap();

		let key = SumTreeName::DepartmentRequiredFund {
			department_required_fund_id,
			block_number: block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_commit_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn get_vote_period_end_block(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let block_number =
			Self::get_block_number_of_schelling_game(department_required_fund_id).unwrap();

		let key = SumTreeName::DepartmentRequiredFund {
			department_required_fund_id,
			block_number: block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_vote_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn selected_as_juror(
		department_required_fund_id: DepartmentRequiredFundId,
		who: T::AccountId,
	) -> bool {
		let block_number =
			Self::get_block_number_of_schelling_game(department_required_fund_id).unwrap();

		let key = SumTreeName::DepartmentRequiredFund {
			department_required_fund_id,
			block_number: block_number.clone(),
		};

		let result = T::SchellingGameSharedSource::selected_as_juror_helper_link(key, who);
		result
	}

	// End block code end
}
