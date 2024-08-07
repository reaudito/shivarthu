use super::*;

// 6 sec (1 block)
// 3 days (43200), 10 days (144000)
// 15 mins (150)
// 5 mins (50)
// 8 mins (80)

impl<T: Config> PhaseData<T> {
	pub fn new(
		evidence_length: BlockNumberOf<T>,
		end_of_staking_time: BlockNumberOf<T>,
		staking_length: BlockNumberOf<T>,
		drawing_length: BlockNumberOf<T>,
		commit_length: BlockNumberOf<T>,
		vote_length: BlockNumberOf<T>,
		appeal_length: BlockNumberOf<T>,
		max_draws: u64,
		min_number_juror_staked: u64,
		min_juror_stake: BalanceOf<T>,
		juror_incentives: (u64, u64),
	) -> Self {
		PhaseData {
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
		}
	}

	pub fn default() -> Self {
		PhaseData {
			evidence_length: 144000u64.saturated_into::<BlockNumberOf<T>>(),
			end_of_staking_time: 144000u64.saturated_into::<BlockNumberOf<T>>(),
			staking_length: 144000u64.saturated_into::<BlockNumberOf<T>>(),
			drawing_length: 144000u64.saturated_into::<BlockNumberOf<T>>(),
			commit_length: 144000u64.saturated_into::<BlockNumberOf<T>>(),
			vote_length: 144000u64.saturated_into::<BlockNumberOf<T>>(),
			appeal_length: 144000u64.saturated_into::<BlockNumberOf<T>>(),
			max_draws: 30,
			min_number_juror_staked: 50,
			min_juror_stake: 1000u64.saturated_into::<BalanceOf<T>>(),
			juror_incentives: (1000, 1000),
		}
	}

	pub fn create_with_data(
		block_length: u64,
		max_draws: u64,
		min_number_juror_staked: u64,
		min_juror_stake: u64,
		juror_incentives: (u64, u64),
	) -> Self {
		let block_length = block_length.saturated_into::<BlockNumberOf<T>>();
		let min_juror_stake = min_juror_stake.saturated_into::<BalanceOf<T>>();
		PhaseData {
			evidence_length: block_length,
			end_of_staking_time: block_length,
			staking_length: block_length,
			drawing_length: block_length,
			commit_length: block_length,
			vote_length: block_length,
			appeal_length: block_length,
			max_draws,
			min_number_juror_staked,
			min_juror_stake,
			juror_incentives,
		}
	}

	pub fn create_phase_with_all_data(
		evidence_length: u64,
		end_of_staking_time: u64,
		staking_length: u64,
		drawing_length: u64,
		commit_length: u64,
		vote_length: u64,
		appeal_length: u64,
		max_draws: u64,
		min_number_juror_staked: u64,
		min_juror_stake: u64,
		juror_incentives: (u64, u64),
	) -> Self {
		let evidence_length = evidence_length.saturated_into::<BlockNumberOf<T>>();
		let end_of_staking_time = end_of_staking_time.saturated_into::<BlockNumberOf<T>>();
		let staking_length = staking_length.saturated_into::<BlockNumberOf<T>>();
		let drawing_length = drawing_length.saturated_into::<BlockNumberOf<T>>();
		let commit_length = commit_length.saturated_into::<BlockNumberOf<T>>();
		let vote_length = vote_length.saturated_into::<BlockNumberOf<T>>();
		let appeal_length = appeal_length.saturated_into::<BlockNumberOf<T>>();

		let min_juror_stake = min_juror_stake.saturated_into::<BalanceOf<T>>();
		PhaseData {
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
		}
	}
}
