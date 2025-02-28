use crate::*;

use trait_schelling_game_shared::SchellingGameSharedLink;

impl<T: Config> SchellingGameSharedLink for Pallet<T> {
	type SumTreeName = SumTreeNameType<T>;
	type SchellingGameType = SchellingGameType;
	type BlockNumber = BlockNumberOf<T>;
	type AccountId = AccountIdOf<T>;
	type Balance = BalanceOf<T>;
	type RangePoint = RangePoint;
	type Period = Period;
	type PhaseData = PhaseDataOf<T>;
	type WinningDecision = WinningDecision;
	type JurorGameResult = JurorGameResult;

	fn create_phase_data(
		block_length: u64,
		max_draws: u64,
		min_number_juror_staked: u64,
		min_juror_stake: u64,
		juror_incentives: (u64, u64),
	) -> Self::PhaseData {
		Self::create_phase_data(
			block_length,
			max_draws,
			min_number_juror_staked,
			min_juror_stake,
			juror_incentives,
		)
	}

	fn create_phase_with_all_data(
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
	) -> Self::PhaseData {
		Self::create_phase_with_all_data(
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

	/// Get the Period
	fn get_period_link(key: Self::SumTreeName) -> Option<Period> {
		Self::get_period(key)
	}

	/// Set `PeriodName` to `Period::Evidence`
	/// Called with submission of `Evidence` stake e.g. Profile stake
	/// Also set `EvidenceStartTime`    
	fn set_to_evidence_period_link(
		key: Self::SumTreeName,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::set_to_evidence_period(key, now)
	}

	/// Create a sortition sum tree   
	fn create_tree_helper_link(key: Self::SumTreeName, k: u64) -> DispatchResult {
		Self::create_tree_link_helper(key, k)
	}

	/// Check `Period` is `Evidence`, and change it to `Staking`   
	/// It is called with function that submits challenge stake after `end_block` of evidence period  
	/// Checks evidence period is over
	#[doc=include_str!("docimage/set_to_staking_period_1.svg")]
	/// ```ignore
	/// if time >= block_time.min_short_block_length {
	///        // change `Period` to `Staking`
	///  }
	/// ```
	fn set_to_staking_period_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::set_to_staking_period(key, phase_data, now)
	}

	fn ensure_time_for_staking_over_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::ensure_time_for_staking_over(key, phase_data, now)
	}

	fn set_to_staking_period_pe_link(
		key: Self::SumTreeName,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::set_to_staking_period_pe(key, now)
	}

	/// Change the `Period`
	///    
	/// `Period::Staking` to `Period::Drawing`
	#[doc=include_str!("docimage/change_period_link_1.svg")]
	/// ```ignore
	/// if now >= min_long_block_length + staking_start_time {
	///   // Change `Period::Staking` to `Period::Drawing`   
	/// }
	/// ```
	///
	///  `Period::Drawing` to `Period::Commit`   
	/// When maximum juror are drawn   
	///  
	/// `Period::Commit` to `Period::Vote`       
	/// ```ignore
	/// if now >= min_long_block_length + commit_start_time {
	///   // Change `Period::Commit` to `Period::Vote`  
	/// }
	/// ```
	///
	/// `Period::Vote` to `Period::Execution`   
	/// ```ignore
	/// if now >= min_long_block_length + vote_start_time {
	///   // Change `Period::Vote` to `Period::Execution`   
	/// }
	/// ```   
	fn change_period_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::change_period(key, phase_data, now)
	}

	/// Apply Jurors      
	/// Ensure `Period` is `Staking`      
	/// Slash the stake.   
	/// Store the stake on sortition sum tree if doesn't exists.   
	fn apply_jurors_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		who: Self::AccountId,
		stake: Self::Balance,
	) -> DispatchResult {
		Self::apply_jurors_helper(key, phase_data, who, stake)
	}

	fn has_user_staked(
		key: Self::SumTreeName,
		who: Self::AccountId,

	) -> bool {
		Self::has_user_staked(key, who)
	}

	fn user_staked_value(key: Self::SumTreeName,
		who: Self::AccountId) -> u64 {
			Self::user_staked_value(key, who)
		}

	/// Draw Jurors  
	/// Ensure `Period` is `Drawing`  
	/// `iterations` is number of jurors drawn per call  
	/// Ensure total draws `draws_in_round` is less than `max_draws`
	fn draw_jurors_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		iterations: u64,
	) -> DispatchResult {
		Self::draw_jurors_helper(key, phase_data, iterations)
	}

	/// Unstake those who are not drawn as jurors   
	/// They can withdraw their stake   
	fn unstaking_helper_link(key: Self::SumTreeName, who: Self::AccountId) -> DispatchResult {
		Self::unstaking_helper(key, who)
	}

	/// Commit vote   
	fn commit_vote_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		vote_commit: [u8; 32],
	) -> DispatchResult {
		Self::commit_vote_helper(key, who, vote_commit)
	}

	/// Reveal vote   
	/// There are two vote choices 0 or 1  
	fn reveal_vote_two_choice_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		choice: u128,
		salt: Vec<u8>,
	) -> DispatchResult {
		Self::reveal_vote_two_choice_helper(key, who, choice, salt)
	}
	/// Distribute incentives for two choices        
	/// Winner gets `stake` + `winning_incentives`      
	/// If decision is draw, jurors receive their `stake`    
	/// Lost jurors gets `stake * 3/4`   
	/// When they receive their incentives, their accountid is stored in `JurorsIncentiveDistributedAccounts`        
	fn get_incentives_two_choice_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		who: Self::AccountId,
	) -> DispatchResult {
		Self::get_incentives_two_choice_helper(key, phase_data, who)
	}

	/// Blocks left for ending evidence period
	/// When evidence time ends, you can submit the challenge stake    
	/// `start_block_number` evidence start time which you will get from `EvidenceStartTime`    
	fn get_evidence_period_end_block_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_evidence_period_end_block_helper(key, phase_data, now)
	}

	/// Blocks left for ending staking period  
	fn get_staking_period_end_block_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_staking_period_end_block_helper(key, phase_data, now)
	}

	/// Return true when drawing period is over, otherwise false   
	fn get_drawing_period_end_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
	) -> (u64, u64, bool) {
		Self::get_drawing_period_end_helper(key, phase_data)
	}

	/// Blocks left for ending drawing period
	fn get_commit_period_end_block_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_commit_period_end_block_helper(key, phase_data, now)
	}

	/// Blocks left for ending vote period
	fn get_vote_period_end_block_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_vote_period_end_block_helper(key, phase_data, now)
	}

	/// Check if `AccountId` is selected as juror
	fn selected_as_juror_helper_link(key: Self::SumTreeName, who: Self::AccountId) -> bool {
		Self::selected_as_juror_helper(key, who)
	}

	/// Commit vote for score schelling game
	fn commit_vote_for_score_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		vote_commit: [u8; 32],
	) -> DispatchResult {
		Self::commit_vote_for_score_helper(key, who, vote_commit)
	}

	/// Reveal vote for score schelling game
	fn reveal_vote_score_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		choice: i64,
		salt: Vec<u8>,
	) -> DispatchResult {
		Self::reveal_vote_score_helper(key, who, choice, salt)
	}

	/// Distribute incentives to all score schelling game jurors
	fn get_incentives_score_schelling_helper_link(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
		range_point: Self::RangePoint,
	) -> DispatchResult {
		Self::get_incentives_score_schelling_helper(key, phase_data, range_point)
	}

	/// Get new mean in score schelling game
	fn get_mean_value_link(key: Self::SumTreeName) -> Result<i64, DispatchError> {
		Self::get_mean_value(key)
	}

	/// Distribute incentives to all two choice shelling game jurors
	fn get_all_incentives_two_choice_helper(
		key: Self::SumTreeName,
		phase_data: Self::PhaseData,
	) -> DispatchResult {
		Self::get_all_incentives_two_choice_helper(key, phase_data)
	}

	fn get_drawn_jurors(key: Self::SumTreeName) -> Vec<(Self::AccountId, u64)> {
		Self::drawn_jurors(key)
	}

	fn get_winning_decision_value(
		key: Self::SumTreeName,
	) -> Result<WinningDecision, DispatchError> {
		Self::get_winning_decision_value(key)
	}

	fn get_result_of_juror(
		key: Self::SumTreeName,
		who: Self::AccountId,
	) -> Result<(JurorGameResult, u64), DispatchError> {
		Self::get_result_of_juror(key, who)
	}

	fn get_result_of_juror_score(
		key: Self::SumTreeName,
		who: Self::AccountId,
		range_point: Self::RangePoint,
	) -> Result<(JurorGameResult, u64), DispatchError> {
		Self::get_result_of_juror_score(key, who, range_point)
	}

	fn set_new_mean_value(key: Self::SumTreeName) -> DispatchResult {
		Self::set_new_mean_value(key)
	}

	fn add_to_incentives_count(key: Self::SumTreeName, who: Self::AccountId) -> DispatchResult {
		Self::add_to_incentives_count(key, who)
	}
}
