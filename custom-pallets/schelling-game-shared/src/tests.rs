use crate::{
    mock::*,
    types::{JurorGameResult, Period, PhaseData, RangePoint, SchellingGameType},
    Error,
};
use frame_support::{assert_noop, assert_ok};

use pallet_sortition_sum_game::types::SumTreeName;

type CitizenId = u64;

fn return_key_profile(citizen_id: CitizenId) -> SumTreeName<u64, u64> {
    let key = SumTreeName::ProfileValidation {
        citizen_address: citizen_id,
        block_number: 10,
    };
    key
}

fn return_game_type_profile_approval() -> SchellingGameType {
    SchellingGameType::ProfileApproval
}

fn get_the_phase_data() -> PhaseData<Test> {
    let data = PhaseData::create_with_data(50, 5, 3, 100, (100, 100));
    data
}

#[test]
fn evidence_period_not_over_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let phase_data = get_the_phase_data();
        let now2 = now + phase_data.evidence_length - 1;
        assert_noop!(
            TemplateModule::set_to_staking_period(key.clone(), phase_data, now2),
            Error::<Test>::EvidencePeriodNotOver
        );
    });
}

/// 1) Set evidence period
/// 2) Set staking period
/// 3) Create tree
#[test]
fn evidence_period_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let phase_data = get_the_phase_data();
        let now2 = now + phase_data.evidence_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data,
            now2
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
    });
}

/// End of staking period

#[test]
fn end_of_time_staking_period() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let phase_data = get_the_phase_data();
        let now2 = now + phase_data.evidence_length + phase_data.end_of_staking_time - 1;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data,
            now2
        ));
        let phase_data = get_the_phase_data();
        let now2 = now + phase_data.evidence_length + phase_data.end_of_staking_time;
        assert_noop!(
            TemplateModule::set_to_staking_period(key.clone(), phase_data, now2),
            Error::<Test>::PeriodIsNotEvidence
        );
    });
}

/// Check time for staking over
#[test]
fn check_time_for_staking_not_over_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let phase_data = get_the_phase_data();
        let now2 = now + phase_data.evidence_length + phase_data.end_of_staking_time - 1;
        assert_noop!(
            TemplateModule::ensure_time_for_staking_over(key.clone(), phase_data, now2),
            Error::<Test>::TimeForStakingNotOver
        );
        let phase_data = get_the_phase_data();
        let now = now + phase_data.evidence_length + phase_data.end_of_staking_time;
        assert_ok!(TemplateModule::ensure_time_for_staking_over(
            key.clone(),
            phase_data,
            now
        ));
    });
}

#[test]
fn apply_juror() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let phase_data = get_the_phase_data();
        let now2 = now + phase_data.evidence_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            now2
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
    });
}

#[test]
fn challenger_win_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let phase_data = get_the_phase_data();

        let staking_start_time = now + phase_data.evidence_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            staking_start_time
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
        let new_now = staking_start_time + phase_data.staking_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Drawing), period);
        assert_ok!(TemplateModule::draw_jurors_helper(
            key.clone(),
            phase_data.clone(),
            5
        ));
        let draws_in_round = TemplateModule::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);
        let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let balance = Balances::free_balance(5);
        assert_eq!(299500, balance);
        assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
        let balance = Balances::free_balance(5);
        assert_eq!(300000, balance);
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 4, hash));
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 7, hash));
        let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 13, hash));
        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 14, hash));
        let hash = sp_io::hashing::keccak_256("0salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 15, hash));
        let commit_start_time = TemplateModule::commit_start_time(key.clone());
        let new_now = commit_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Vote), period);
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            4,
            1,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            7,
            1,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            13,
            1,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            14,
            1,
            "salt4".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            15,
            0,
            "salt5".as_bytes().to_vec()
        ));
        let decision = TemplateModule::decision_count(key.clone());
        assert_eq!((1, 4), decision);
        let vote_start_time = TemplateModule::vote_start_time(key.clone());
        let new_now = vote_start_time + phase_data.vote_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Execution), period);

        let balance = Balances::free_balance(4);
        assert_eq!(299600, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            4
        ));
        let balance = Balances::free_balance(4);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(7);
        // println!("{:?}", balance);
        assert_eq!(299300, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            7
        ));
        let balance = Balances::free_balance(7);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(13);
        assert_eq!(298700, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            13
        ));
        let balance = Balances::free_balance(13);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(298600, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            14
        ));
        let balance = Balances::free_balance(14);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(15);
        assert_eq!(298500, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            15
        ));
        let balance = Balances::free_balance(15);
        assert_eq!(299625, balance);
    });
}

#[test]
fn challenger_win_value_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let phase_data = get_the_phase_data();

        let staking_start_time = now + phase_data.evidence_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            staking_start_time
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
        let new_now = staking_start_time + phase_data.staking_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Drawing), period);
        assert_ok!(TemplateModule::draw_jurors_helper(
            key.clone(),
            phase_data.clone(),
            5
        ));
        let draws_in_round = TemplateModule::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);
        let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let balance = Balances::free_balance(5);
        assert_eq!(299500, balance);
        assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
        let balance = Balances::free_balance(5);
        assert_eq!(300000, balance);
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 4, hash));
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 7, hash));
        let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 13, hash));
        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 14, hash));
        let hash = sp_io::hashing::keccak_256("0salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 15, hash));
        let commit_start_time = TemplateModule::commit_start_time(key.clone());
        let new_now = commit_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Vote), period);
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            4,
            1,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            7,
            1,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            13,
            1,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            14,
            1,
            "salt4".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            15,
            0,
            "salt5".as_bytes().to_vec()
        ));
        let decision = TemplateModule::decision_count(key.clone());
        assert_eq!((1, 4), decision);
        let vote_start_time = TemplateModule::vote_start_time(key.clone());
        let new_now = vote_start_time + phase_data.vote_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Execution), period);
        let result_stake = TemplateModule::get_result_of_juror(key.clone(), 4);
        let (result, _) = result_stake.unwrap();
        assert_eq!(result, JurorGameResult::Won);
        let result_stake = TemplateModule::get_result_of_juror(key.clone(), 7);
        let (result, _) = result_stake.unwrap();
        assert_eq!(result, JurorGameResult::Won);
        let result_stake = TemplateModule::get_result_of_juror(key.clone(), 13);
        let (result, _) = result_stake.unwrap();
        assert_eq!(result, JurorGameResult::Won);
        let result_stake = TemplateModule::get_result_of_juror(key.clone(), 14);
        let (result, _) = result_stake.unwrap();
        assert_eq!(result, JurorGameResult::Won);
        let result_stake = TemplateModule::get_result_of_juror(key.clone(), 15);
        let (result, _) = result_stake.unwrap();
        assert_eq!(result, JurorGameResult::Lost);
    });
}

#[test]
fn challenger_win_test_jurors_incentive_in_one_go() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let _game_type = return_game_type_profile_approval();

        // let min_short_block_length = return_min_short_block_length();
        // let min_long_block_length = return_min_long_block_length();

        let phase_data = get_the_phase_data();

        let staking_start_time = now + phase_data.staking_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            staking_start_time
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
        let new_now = staking_start_time + phase_data.staking_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Drawing), period);
        assert_ok!(TemplateModule::draw_jurors_helper(
            key.clone(),
            phase_data.clone(),
            5
        ));
        let draws_in_round = TemplateModule::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);
        let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let balance = Balances::free_balance(5);
        assert_eq!(299500, balance);
        assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
        let balance = Balances::free_balance(5);
        assert_eq!(300000, balance);
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 4, hash));
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 7, hash));
        let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 13, hash));
        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 14, hash));
        let hash = sp_io::hashing::keccak_256("0salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 15, hash));
        let commit_start_time = TemplateModule::commit_start_time(key.clone());
        let new_now = commit_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Vote), period);
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            4,
            1,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            7,
            1,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            13,
            1,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            14,
            1,
            "salt4".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            15,
            0,
            "salt5".as_bytes().to_vec()
        ));
        let decision = TemplateModule::decision_count(key.clone());
        assert_eq!((1, 4), decision);
        let vote_start_time = TemplateModule::vote_start_time(key.clone());
        let new_now = vote_start_time + phase_data.vote_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Execution), period);
        let balance = Balances::free_balance(4);
        assert_eq!(299600, balance);
        let balance = Balances::free_balance(7);
        // println!("{:?}", balance);
        assert_eq!(299300, balance);
        let balance = Balances::free_balance(13);
        assert_eq!(298700, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(298600, balance);
        let balance = Balances::free_balance(15);
        assert_eq!(298500, balance);
        assert_ok!(TemplateModule::get_all_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone()
        ));
        let balance = Balances::free_balance(4);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(7);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(13);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(15);
        assert_eq!(299625, balance);
    });
}

#[test]
fn challenger_lost_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        // let game_type = return_game_type_profile_approval();
        let phase_data = get_the_phase_data();

        // let min_short_block_length = return_min_short_block_length();
        // let min_long_block_length = return_min_long_block_length();
        let staking_start_time = now + phase_data.staking_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            staking_start_time
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
        let new_now = staking_start_time + phase_data.staking_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Drawing), period);
        assert_ok!(TemplateModule::draw_jurors_helper(
            key.clone(),
            phase_data.clone(),
            5
        ));
        let draws_in_round = TemplateModule::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);
        let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let balance = Balances::free_balance(5);
        assert_eq!(299500, balance);
        assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
        let balance = Balances::free_balance(5);
        assert_eq!(300000, balance);
        let hash = sp_io::hashing::keccak_256("0salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 4, hash));
        let hash = sp_io::hashing::keccak_256("0salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 7, hash));
        let hash = sp_io::hashing::keccak_256("0salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 13, hash));
        let hash = sp_io::hashing::keccak_256("0salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 14, hash));
        let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 15, hash));
        let commit_start_time = TemplateModule::commit_start_time(key.clone());
        let new_now = commit_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Vote), period);
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            4,
            0,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            7,
            0,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            13,
            0,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            14,
            0,
            "salt4".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
            key.clone(),
            15,
            1,
            "salt5".as_bytes().to_vec()
        ));
        let decision = TemplateModule::decision_count(key.clone());
        assert_eq!((4, 1), decision);
        let vote_start_time = TemplateModule::vote_start_time(key.clone());
        let new_now = vote_start_time + phase_data.vote_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Execution), period);

        let balance = Balances::free_balance(4);
        assert_eq!(299600, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            4
        ));
        let balance = Balances::free_balance(4);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(7);
        // println!("{:?}", balance);
        assert_eq!(299300, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            7
        ));
        let balance = Balances::free_balance(7);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(13);
        assert_eq!(298700, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            13
        ));
        let balance = Balances::free_balance(13);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(298600, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            14
        ));
        let balance = Balances::free_balance(14);
        assert_eq!(300025, balance);
        let balance = Balances::free_balance(15);
        assert_eq!(298500, balance);
        assert_ok!(TemplateModule::get_incentives_two_choice_helper(
            key.clone(),
            phase_data.clone(),
            15
        ));
        let balance = Balances::free_balance(15);
        assert_eq!(299625, balance);
    });
}

#[test]
fn score_schelling_game_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let _game_type = return_game_type_profile_approval();
        // let min_short_block_length = return_min_short_block_length();
        // let min_long_block_length = return_min_long_block_length();
        let phase_data = get_the_phase_data();

        let staking_start_time = now + phase_data.staking_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            staking_start_time
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
        let new_now = staking_start_time + phase_data.staking_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Drawing), period);
        assert_ok!(TemplateModule::draw_jurors_helper(
            key.clone(),
            phase_data.clone(),
            5
        ));
        let draws_in_round = TemplateModule::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);
        let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let balance = Balances::free_balance(5);
        assert_eq!(299500, balance);
        assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
        let balance = Balances::free_balance(5);
        assert_eq!(300000, balance);
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            4,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            7,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("5salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            13,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            14,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("7salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            15,
            hash
        ));
        let commit_start_time = TemplateModule::commit_start_time(key.clone());
        let new_now = commit_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Vote), period);
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            4,
            1,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            7,
            1,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            13,
            5,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            14,
            1,
            "salt4".as_bytes().to_vec()
        ));
        assert_noop!(
            TemplateModule::reveal_vote_score_helper(
                key.clone(),
                15,
                8,
                "salt5".as_bytes().to_vec()
            ),
            Error::<Test>::CommitDoesNotMatch
        );
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            15,
            7,
            "salt5".as_bytes().to_vec()
        ));
        let vote_start_time = TemplateModule::vote_start_time(key.clone());
        let new_now = vote_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Execution), period);
        let reveal_score = TemplateModule::reveal_score_values(key.clone());
        assert_eq!(vec![1000, 1000, 5000, 1000, 7000], reveal_score);
        let balance = Balances::free_balance(4);
        assert_eq!(299600, balance);
        let balance = Balances::free_balance(7);
        // println!("{:?}", balance);
        assert_eq!(299300, balance);
        let balance = Balances::free_balance(13);
        assert_eq!(298700, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(298600, balance);
        let balance = Balances::free_balance(15);
        assert_eq!(298500, balance);
        assert_ok!(TemplateModule::get_incentives_score_schelling_helper(
            key.clone(),
            phase_data.clone(),
            RangePoint::ZeroToTen
        ));
        let mean_values = TemplateModule::new_mean_reveal_score(key.clone());
        assert_eq!(2000, mean_values.unwrap());
        let balance = Balances::free_balance(4);
        // println!("{:?}", balance);
        assert_eq!(300033, balance);
        let balance = Balances::free_balance(7);
        assert_eq!(300033, balance);
        let balance = Balances::free_balance(13); // Balance deducted as voted 5
        assert_eq!(299675, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(300033, balance);
        let balance = Balances::free_balance(15); // Balance deducted as voted 7
        assert_eq!(299625, balance);
    });
}

#[test]
fn score_schelling_game_not_reveal_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let _game_type = return_game_type_profile_approval();
        // let min_short_block_length = return_min_short_block_length();
        // let min_long_block_length = return_min_long_block_length();
        let phase_data = get_the_phase_data();

        let staking_start_time = now + phase_data.staking_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            staking_start_time
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
        let new_now = staking_start_time + phase_data.staking_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Drawing), period);
        assert_ok!(TemplateModule::draw_jurors_helper(
            key.clone(),
            phase_data.clone(),
            5
        ));
        let draws_in_round = TemplateModule::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);
        let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let balance = Balances::free_balance(5);
        assert_eq!(299500, balance);
        assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
        let balance = Balances::free_balance(5);
        assert_eq!(300000, balance);
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            4,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            7,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("5salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            13,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            14,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("7salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            15,
            hash
        ));
        let commit_start_time = TemplateModule::commit_start_time(key.clone());
        let new_now = commit_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Vote), period);
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            4,
            1,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            7,
            1,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            13,
            5,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            14,
            1,
            "salt4".as_bytes().to_vec()
        ));
        assert_noop!(
            TemplateModule::reveal_vote_score_helper(
                key.clone(),
                15,
                8,
                "salt5".as_bytes().to_vec()
            ),
            Error::<Test>::CommitDoesNotMatch
        );
        // assert_ok!(TemplateModule::reveal_vote_score_helper(
        // 	key.clone(),
        // 	15,
        // 	7,
        // 	"salt5".as_bytes().to_vec()
        // ));
        let vote_start_time = TemplateModule::vote_start_time(key.clone());
        let new_now = vote_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Execution), period);
        let reveal_score = TemplateModule::reveal_score_values(key.clone());
        assert_eq!(vec![1000, 1000, 5000, 1000], reveal_score);
        let balance = Balances::free_balance(4);
        assert_eq!(299600, balance);
        let balance = Balances::free_balance(7);
        // println!("{:?}", balance);
        assert_eq!(299300, balance);
        let balance = Balances::free_balance(13);
        assert_eq!(298700, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(298600, balance);
        let balance = Balances::free_balance(15);
        assert_eq!(298500, balance);
        assert_ok!(TemplateModule::get_incentives_score_schelling_helper(
            key.clone(),
            phase_data.clone(),
            RangePoint::ZeroToTen
        ));
        let mean_values = TemplateModule::new_mean_reveal_score(key.clone());
        assert_eq!(1000, mean_values.unwrap());
        let balance = Balances::free_balance(4);
        // println!("{:?}", balance);
        assert_eq!(300033, balance);
        let balance = Balances::free_balance(7);
        assert_eq!(300033, balance);
        let balance = Balances::free_balance(13); // Balance deducted as voted 5
        assert_eq!(299675, balance);
        let balance = Balances::free_balance(14);
        assert_eq!(300033, balance);
        let balance = Balances::free_balance(15); // Balance deducted as voted 7
        assert_eq!(298500, balance);
    });
}

#[test]
fn score_schelling_game_value_test() {
    new_test_ext().execute_with(|| {
        let key = return_key_profile(0);
        let now = 10;
        assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
        assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
        let _game_type = return_game_type_profile_approval();
        // let min_short_block_length = return_min_short_block_length();
        // let min_long_block_length = return_min_long_block_length();
        let phase_data = get_the_phase_data();

        let staking_start_time = now + phase_data.staking_length;
        assert_ok!(TemplateModule::set_to_staking_period(
            key.clone(),
            phase_data.clone(),
            staking_start_time
        ));
        // Create tree
        assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
        // Check the period is staking
        let period = TemplateModule::get_period(key.clone());
        // println!("{:?}", period);
        assert_eq!(Some(Period::Staking), period);
        // Applyjuror
        for j in 4..30 {
            assert_ok!(TemplateModule::apply_jurors_helper(
                key.clone(),
                phase_data.clone(),
                j,
                j * 100
            ));
        }
        let new_now = staking_start_time + phase_data.staking_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Drawing), period);
        assert_ok!(TemplateModule::draw_jurors_helper(
            key.clone(),
            phase_data.clone(),
            5
        ));
        let draws_in_round = TemplateModule::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);
        let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let balance = Balances::free_balance(5);
        assert_eq!(299500, balance);
        assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
        let balance = Balances::free_balance(5);
        assert_eq!(300000, balance);
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            4,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            7,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("5salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            13,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            14,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("7salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote_for_score_helper(
            key.clone(),
            15,
            hash
        ));
        let commit_start_time = TemplateModule::commit_start_time(key.clone());
        let new_now = commit_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Vote), period);
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            4,
            1,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            7,
            1,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            13,
            5,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            14,
            1,
            "salt4".as_bytes().to_vec()
        ));
        assert_noop!(
            TemplateModule::reveal_vote_score_helper(
                key.clone(),
                15,
                8,
                "salt5".as_bytes().to_vec()
            ),
            Error::<Test>::CommitDoesNotMatch
        );
        assert_ok!(TemplateModule::reveal_vote_score_helper(
            key.clone(),
            15,
            7,
            "salt5".as_bytes().to_vec()
        ));
        let vote_start_time = TemplateModule::vote_start_time(key.clone());
        let new_now = vote_start_time + phase_data.commit_length;
        assert_ok!(TemplateModule::change_period(
            key.clone(),
            phase_data.clone(),
            new_now.clone()
        ));
        let period = TemplateModule::get_period(key.clone());
        assert_eq!(Some(Period::Execution), period);
        let reveal_score = TemplateModule::reveal_score_values(key.clone());
        assert_eq!(vec![1000, 1000, 5000, 1000, 7000], reveal_score);
        assert_ok!(TemplateModule::set_new_mean_value(key.clone()));
        let mean_values = TemplateModule::new_mean_reveal_score(key.clone());
        assert_eq!(2000, mean_values.unwrap());
        let result_stake =
            TemplateModule::get_result_of_juror_score(key.clone(), 4, RangePoint::ZeroToTen);
        let (result, _) = result_stake.unwrap();
        assert_eq!(result, JurorGameResult::Won);
        let result_stake =
            TemplateModule::get_result_of_juror_score(key.clone(), 7, RangePoint::ZeroToTen);
        let (result, _) = result_stake.unwrap();

        assert_eq!(result, JurorGameResult::Won);
        let result_stake =
            TemplateModule::get_result_of_juror_score(key.clone(), 13, RangePoint::ZeroToTen);
        let (result, _) = result_stake.unwrap();

        assert_eq!(result, JurorGameResult::Lost);
        let result_stake =
            TemplateModule::get_result_of_juror_score(key.clone(), 14, RangePoint::ZeroToTen);
        let (result, _) = result_stake.unwrap();

        assert_eq!(result, JurorGameResult::Won);
        let result_stake =
            TemplateModule::get_result_of_juror_score(key.clone(), 15, RangePoint::ZeroToTen);
        let (result, _) = result_stake.unwrap();

        assert_eq!(result, JurorGameResult::Lost);
    });
}
