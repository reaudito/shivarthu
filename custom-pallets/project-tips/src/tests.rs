use crate::types::{Incentives, TippingName};
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_schelling_game_shared::types::Period;
use pallet_sortition_sum_game::types::SumTreeName;
use pallet_support::Content;
use pallet_support::WhenDetails;

#[test]
fn check_balance_on_staking() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);
        let tipping_name = TippingName::SmallTipper;
        let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
        let max_tipping_value = tipping_value.max_tipping_value;
        let stake_required = tipping_value.stake_required;
        let funding_needed = max_tipping_value - 100;
        let content: Content = Content::IPFS(
            "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
                .as_bytes()
                .to_vec(),
        );
        assert_ok!(ProjectTips::create_project(
            RuntimeOrigin::signed(1),
            2,
            content.clone(),
            tipping_name,
            funding_needed
        ));

        System::assert_last_event(
            Event::ProjectCreated {
                account: 1,
                project_id: 1,
            }
            .into(),
        );

        let balance = Balances::free_balance(1);

        assert_ok!(ProjectTips::apply_staking_period(
            RuntimeOrigin::signed(1),
            1
        ));

        let after_balance = Balances::free_balance(1);

        assert_eq!(after_balance, balance - stake_required);
    });
}

#[test]
fn check_create_project_function() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let tipping_name = TippingName::SmallTipper;
        let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
        let max_tipping_value = tipping_value.max_tipping_value;
        let funding_needed = max_tipping_value - 100;
        let content: Content = Content::IPFS(
            "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
                .as_bytes()
                .to_vec(),
        );
        assert_ok!(ProjectTips::create_project(
            RuntimeOrigin::signed(1),
            2,
            content.clone(),
            tipping_name,
            funding_needed
        ));

        System::assert_last_event(
            Event::ProjectCreated {
                account: 1,
                project_id: 1,
            }
            .into(),
        );

        let next_project_id = ProjectTips::next_project_id();

        assert_eq!(2, next_project_id);

        let funding_needed = max_tipping_value + 100;

        assert_noop!(
            ProjectTips::create_project(
                RuntimeOrigin::signed(1),
                2,
                content,
                tipping_name,
                funding_needed
            ),
            Error::<Test>::FundingMoreThanTippingValue
        );
    });
}

#[test]
fn check_apply_staking_period_function() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ProjectTips::apply_staking_period(RuntimeOrigin::signed(1), 2),
            Error::<Test>::ProjectDontExists
        );

        let tipping_name = TippingName::SmallTipper;
        let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
        let max_tipping_value = tipping_value.max_tipping_value;
        let funding_needed = max_tipping_value - 100;
        let content: Content = Content::IPFS(
            "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
                .as_bytes()
                .to_vec(),
        );
        assert_ok!(ProjectTips::create_project(
            RuntimeOrigin::signed(1),
            2,
            content,
            tipping_name,
            funding_needed
        ));

        assert_noop!(
            ProjectTips::apply_staking_period(RuntimeOrigin::signed(3), 1),
            Error::<Test>::ProjectCreatorDontMatch
        );

        assert_ok!(ProjectTips::apply_staking_period(
            RuntimeOrigin::signed(1),
            1
        ));

        System::assert_last_event(
            Event::StakinPeriodStarted {
                project_id: 1,
                block_number: 1,
            }
            .into(),
        );
        System::set_block_number(5);
        assert_noop!(
            ProjectTips::apply_staking_period(RuntimeOrigin::signed(1), 1),
            Error::<Test>::ProjectIdStakingPeriodAlreadySet
        );
    });
}

#[test]
fn schelling_game_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let tipping_name = TippingName::SmallTipper;
        let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
        let max_tipping_value = tipping_value.max_tipping_value;
        let stake_required = tipping_value.stake_required;
        let funding_needed = max_tipping_value - 100;
        let content: Content = Content::IPFS(
            "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
                .as_bytes()
                .to_vec(),
        );
        assert_ok!(ProjectTips::create_project(
            RuntimeOrigin::signed(1),
            2,
            content,
            tipping_name,
            funding_needed
        ));

        let balance = Balances::free_balance(1);

        assert_ok!(ProjectTips::apply_staking_period(
            RuntimeOrigin::signed(1),
            1
        ));

        let after_balance = Balances::free_balance(1);

        assert_eq!(after_balance, balance - stake_required);

        let phase_data = ProjectTips::get_phase_data();

        let balance = Balances::free_balance(29);
        assert_eq!(3000000, balance);
        for j in 4..30 {
            assert_ok!(ProjectTips::apply_jurors(
                RuntimeOrigin::signed(j),
                1,
                j * 100
            ));
        }

        let balance = Balances::free_balance(29);
        assert_eq!(3000000 - 29 * 100, balance);

        assert_noop!(
            ProjectTips::draw_jurors(RuntimeOrigin::signed(5), 1, 5),
            <pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
        );

        assert_noop!(
            ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
            <pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
        );

        System::set_block_number(1 + phase_data.staking_length);

        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        assert_ok!(ProjectTips::draw_jurors(RuntimeOrigin::signed(5), 1, 5));

        let key = SumTreeName::ProjectTips {
            project_id: 1,
            block_number: 1,
        };

        let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);

        let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );

        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        let period = SchellingGameShared::get_period(key.clone());

        assert_eq!(Some(Period::Commit), period);

        let balance: u64 = Balances::free_balance(5);
        assert_eq!(3000000 - 5 * 100, balance);
        assert_ok!(ProjectTips::unstaking(RuntimeOrigin::signed(5), 1));
        let balance = Balances::free_balance(5);
        assert_eq!(3000000, balance);

        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_noop!(
            ProjectTips::commit_vote(RuntimeOrigin::signed(6), 1, hash),
            <pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
        );
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(4), 1, hash));

        // You can replace vote within the commit period.
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(4), 1, hash));

        let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(7), 1, hash));

        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(13), 1, hash));

        let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(14), 1, hash));

        let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(15), 1, hash));

        assert_noop!(
            ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
            <pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
        );
        System::set_block_number(
            phase_data.evidence_length + 1 + phase_data.staking_length + phase_data.commit_length,
        );
        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        assert_noop!(
            ProjectTips::reveal_vote(RuntimeOrigin::signed(4), 1, 2, "salt2".as_bytes().to_vec()),
            <pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
        );

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(4),
            1,
            1,
            "salt2".as_bytes().to_vec()
        ));

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(7),
            1,
            1,
            "salt3".as_bytes().to_vec()
        ));

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(13),
            1,
            1,
            "salt4".as_bytes().to_vec()
        ));

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(14),
            1,
            1,
            "salt5".as_bytes().to_vec()
        ));

        assert_noop!(
            ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
            <pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
        );
        System::set_block_number(
            phase_data.evidence_length
                + 1
                + phase_data.staking_length
                + phase_data.commit_length
                + phase_data.vote_length,
        );
        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        // assert_noop!(
        // 	ProjectTips::get_incentives(RuntimeOrigin::signed(15), 1),
        // 	<pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
        // );
        // let balance: u64 = Balances::free_balance(14);
        // assert_eq!(3000000 - 14 * 100, balance);
        // assert_ok!(ProjectTips::get_incentives(RuntimeOrigin::signed(14), 1));
        // let balance: u64 = Balances::free_balance(14);
        // assert_eq!(300025, balance);
    })
}

#[test]
fn schelling_game_incentives_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let tipping_name = TippingName::SmallTipper;
        let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
        let max_tipping_value = tipping_value.max_tipping_value;
        let stake_required = tipping_value.stake_required;
        let funding_needed = max_tipping_value - 100;
        let content: Content = Content::IPFS(
            "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
                .as_bytes()
                .to_vec(),
        );
        assert_ok!(ProjectTips::create_project(
            RuntimeOrigin::signed(1),
            2,
            content,
            tipping_name,
            funding_needed
        ));

        let balance = Balances::free_balance(1);

        assert_ok!(ProjectTips::apply_staking_period(
            RuntimeOrigin::signed(1),
            1
        ));

        let after_balance = Balances::free_balance(1);

        assert_eq!(after_balance, balance - stake_required);

        let phase_data = ProjectTips::get_phase_data();

        let balance = Balances::free_balance(29);
        assert_eq!(3000000, balance);
        for j in 4..30 {
            assert_ok!(ProjectTips::apply_jurors(
                RuntimeOrigin::signed(j),
                1,
                j * 100
            ));
        }

        let balance = Balances::free_balance(29);
        assert_eq!(3000000 - 29 * 100, balance);

        assert_noop!(
            ProjectTips::draw_jurors(RuntimeOrigin::signed(5), 1, 5),
            <pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
        );

        assert_noop!(
            ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
            <pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
        );

        System::set_block_number(1 + phase_data.staking_length);

        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        assert_ok!(ProjectTips::draw_jurors(RuntimeOrigin::signed(5), 1, 5));

        let key = SumTreeName::ProjectTips {
            project_id: 1,
            block_number: 1,
        };

        let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
        assert_eq!(5, draws_in_round);

        let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
        assert_eq!(
            vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
            drawn_jurors
        );

        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        let period = SchellingGameShared::get_period(key.clone());

        assert_eq!(Some(Period::Commit), period);

        let balance: u64 = Balances::free_balance(5);
        assert_eq!(3000000 - 5 * 100, balance);
        assert_ok!(ProjectTips::unstaking(RuntimeOrigin::signed(5), 1));
        let balance = Balances::free_balance(5);
        assert_eq!(3000000, balance);

        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_noop!(
            ProjectTips::commit_vote(RuntimeOrigin::signed(6), 1, hash),
            <pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
        );
        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(4), 1, hash));

        // You can replace vote within the commit period.
        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(4), 1, hash));

        let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(7), 1, hash));

        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(13), 1, hash));

        let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(14), 1, hash));

        let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
        assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(15), 1, hash));

        assert_noop!(
            ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
            <pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
        );
        System::set_block_number(
            phase_data.evidence_length + 1 + phase_data.staking_length + phase_data.commit_length,
        );
        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        assert_noop!(
            ProjectTips::reveal_vote(RuntimeOrigin::signed(4), 1, 2, "salt2".as_bytes().to_vec()),
            <pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
        );

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(4),
            1,
            1,
            "salt2".as_bytes().to_vec()
        ));

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(7),
            1,
            1,
            "salt3".as_bytes().to_vec()
        ));

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(14),
            1,
            1,
            "salt5".as_bytes().to_vec()
        ));

        assert_ok!(ProjectTips::reveal_vote(
            RuntimeOrigin::signed(15),
            1,
            0,
            "salt6".as_bytes().to_vec()
        ));

        assert_noop!(
            ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
            <pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
        );
        System::set_block_number(
            phase_data.evidence_length
                + 1
                + phase_data.staking_length
                + phase_data.commit_length
                + phase_data.vote_length,
        );
        assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));

        assert_noop!(
            ProjectTips::add_incentive_count(RuntimeOrigin::signed(13), 1),
            <pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
        );

        assert_ok!(ProjectTips::add_incentive_count(
            RuntimeOrigin::signed(14),
            1
        ));

        let incentive_count = ProjectTips::incentives_count(14).unwrap();
        // println!("{:?}", incentive_count);
        // Explicitly specify the type of `incentive_count_eq`
        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 1,
            winner: 1,
            loser: 0,
            total_stake: 14 * 100,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        // Your test logic here
        assert_eq!(incentive_count, incentive_count_eq);

        assert_noop!(
            ProjectTips::add_incentive_count(RuntimeOrigin::signed(14), 1),
            <pallet_schelling_game_shared::Error<Test>>::AlreadyIncentivesAdded
        );

        assert_ok!(ProjectTips::add_incentive_count(
            RuntimeOrigin::signed(15),
            1
        ));

        let incentive_count = ProjectTips::incentives_count(15).unwrap();

        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 1,
            winner: 0,
            loser: 1,
            total_stake: 15 * 100,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        assert_eq!(incentive_count, incentive_count_eq);
    })
}

// Play two schelling game to check incentives are updated

fn full_schelling_game_func(who_ask_tipper: u64, start_block_number: u64) {
    let tipping_name = TippingName::SmallTipper;
    let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
    let max_tipping_value = tipping_value.max_tipping_value;
    let stake_required = tipping_value.stake_required;
    let funding_needed = max_tipping_value - 100;
    let content: Content = Content::IPFS(
        "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
            .as_bytes()
            .to_vec(),
    );
    assert_ok!(ProjectTips::create_project(
        RuntimeOrigin::signed(who_ask_tipper),
        2,
        content,
        tipping_name,
        funding_needed
    ));

    let project_ids = ProjectTips::get_projects_from_accounts(who_ask_tipper);
    let project_id = project_ids.last().unwrap();
    let project_id = *project_id;

    let balance = Balances::free_balance(who_ask_tipper);

    assert_ok!(ProjectTips::apply_staking_period(
        RuntimeOrigin::signed(who_ask_tipper),
        project_id
    ));

    let after_balance = Balances::free_balance(who_ask_tipper);

    assert_eq!(after_balance, balance - stake_required);

    let phase_data = ProjectTips::get_phase_data();

    for j in 4..30 {
        assert_ok!(ProjectTips::apply_jurors(
            RuntimeOrigin::signed(j),
            project_id,
            j * 100
        ));
    }

    assert_noop!(
        ProjectTips::draw_jurors(RuntimeOrigin::signed(5), project_id, 5),
        <pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
    );

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
    );

    System::set_block_number(start_block_number + phase_data.staking_length);

    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_ok!(ProjectTips::draw_jurors(
        RuntimeOrigin::signed(5),
        project_id,
        5
    ));

    let key = SumTreeName::ProjectTips {
        project_id: project_id,
        block_number: start_block_number,
    };

    let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
    assert_eq!(5, draws_in_round);

    let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
    assert_eq!(
        vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
        drawn_jurors
    );

    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    let period = SchellingGameShared::get_period(key.clone());

    assert_eq!(Some(Period::Commit), period);

    assert_ok!(ProjectTips::unstaking(RuntimeOrigin::signed(5), project_id));

    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_noop!(
        ProjectTips::commit_vote(RuntimeOrigin::signed(6), project_id, hash),
        <pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
    );
    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(4),
        project_id,
        hash
    ));

    // You can replace vote within the commit period.
    let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(4),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(7),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(13),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(14),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(15),
        project_id,
        hash
    ));

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length,
    );
    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_noop!(
        ProjectTips::reveal_vote(
            RuntimeOrigin::signed(4),
            project_id,
            2,
            "salt2".as_bytes().to_vec()
        ),
        <pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
    );

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(4),
        project_id,
        1,
        "salt2".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(7),
        project_id,
        1,
        "salt3".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(14),
        project_id,
        1,
        "salt5".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(15),
        project_id,
        0,
        "salt6".as_bytes().to_vec()
    ));

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length
            + phase_data.vote_length,
    );
    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_noop!(
        ProjectTips::add_incentive_count(RuntimeOrigin::signed(13), project_id),
        <pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
    );
    assert_ok!(ProjectTips::add_incentive_count(
        RuntimeOrigin::signed(14),
        project_id
    ));
    assert_ok!(ProjectTips::add_incentive_count(
        RuntimeOrigin::signed(15),
        project_id
    ));
}

fn full_schelling_game_func2(who_ask_tipper: u64, start_block_number: u64) {
    let tipping_name = TippingName::SmallTipper;
    let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
    let max_tipping_value = tipping_value.max_tipping_value;
    let stake_required = tipping_value.stake_required;
    let funding_needed = max_tipping_value - 100;
    let content: Content = Content::IPFS(
        "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
            .as_bytes()
            .to_vec(),
    );
    assert_ok!(ProjectTips::create_project(
        RuntimeOrigin::signed(who_ask_tipper),
        2,
        content,
        tipping_name,
        funding_needed
    ));

    let project_ids = ProjectTips::get_projects_from_accounts(who_ask_tipper);
    let project_id = project_ids.last().unwrap();
    let project_id = *project_id;
    // println!("project id {project_id}");

    let balance = Balances::free_balance(who_ask_tipper);

    assert_ok!(ProjectTips::apply_staking_period(
        RuntimeOrigin::signed(who_ask_tipper),
        project_id
    ));

    let after_balance = Balances::free_balance(who_ask_tipper);

    assert_eq!(after_balance, balance - stake_required);

    let phase_data = ProjectTips::get_phase_data();

    for j in 4..30 {
        assert_ok!(ProjectTips::apply_jurors(
            RuntimeOrigin::signed(j),
            project_id,
            j * 100
        ));
    }

    assert_noop!(
        ProjectTips::draw_jurors(RuntimeOrigin::signed(5), project_id, 5),
        <pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
    );

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
    );

    System::set_block_number(start_block_number + phase_data.staking_length);

    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_ok!(ProjectTips::draw_jurors(
        RuntimeOrigin::signed(5),
        project_id,
        5
    ));

    let key = SumTreeName::ProjectTips {
        project_id: project_id,
        block_number: start_block_number,
    };

    let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
    // println!("Draws in round{draws_in_round}");
    assert_eq!(5, draws_in_round);

    let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
    assert_eq!(
        vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
        drawn_jurors
    );

    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    let period = SchellingGameShared::get_period(key.clone());

    assert_eq!(Some(Period::Commit), period);

    assert_ok!(ProjectTips::unstaking(RuntimeOrigin::signed(5), project_id));

    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_noop!(
        ProjectTips::commit_vote(RuntimeOrigin::signed(6), project_id, hash),
        <pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
    );
    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(4),
        project_id,
        hash
    ));

    // You can replace vote within the commit period.
    let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(4),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(7),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(13),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("0salt5".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(14),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt6".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(15),
        project_id,
        hash
    ));

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length,
    );
    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_noop!(
        ProjectTips::reveal_vote(
            RuntimeOrigin::signed(4),
            project_id,
            2,
            "salt2".as_bytes().to_vec()
        ),
        <pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
    );

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(4),
        project_id,
        1,
        "salt2".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(7),
        project_id,
        1,
        "salt3".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(14),
        project_id,
        0,
        "salt5".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(15),
        project_id,
        1,
        "salt6".as_bytes().to_vec()
    ));

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length
            + phase_data.vote_length,
    );
    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_noop!(
        ProjectTips::add_incentive_count(RuntimeOrigin::signed(13), project_id),
        <pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
    );
    assert_ok!(ProjectTips::add_incentive_count(
        RuntimeOrigin::signed(14),
        project_id
    ));
    assert_ok!(ProjectTips::add_incentive_count(
        RuntimeOrigin::signed(15),
        project_id
    ));
}

fn full_schelling_game_func_ask_tipper_defeated(who_ask_tipper: u64, start_block_number: u64) {
    let tipping_name = TippingName::SmallTipper;
    let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
    let max_tipping_value = tipping_value.max_tipping_value;
    let stake_required = tipping_value.stake_required;
    let funding_needed = max_tipping_value - 100;
    let content: Content = Content::IPFS(
        "bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
            .as_bytes()
            .to_vec(),
    );
    assert_ok!(ProjectTips::create_project(
        RuntimeOrigin::signed(who_ask_tipper),
        2,
        content,
        tipping_name,
        funding_needed
    ));

    let project_ids = ProjectTips::get_projects_from_accounts(who_ask_tipper);
    let project_id = project_ids.last().unwrap();
    let project_id = *project_id;

    let balance = Balances::free_balance(who_ask_tipper);

    assert_ok!(ProjectTips::apply_staking_period(
        RuntimeOrigin::signed(who_ask_tipper),
        project_id
    ));

    let after_balance = Balances::free_balance(who_ask_tipper);

    assert_eq!(after_balance, balance - stake_required);

    let phase_data = ProjectTips::get_phase_data();

    for j in 4..30 {
        assert_ok!(ProjectTips::apply_jurors(
            RuntimeOrigin::signed(j),
            project_id,
            j * 100
        ));
    }

    assert_noop!(
        ProjectTips::draw_jurors(RuntimeOrigin::signed(5), project_id, 5),
        <pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
    );

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
    );

    System::set_block_number(start_block_number + phase_data.staking_length);

    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_ok!(ProjectTips::draw_jurors(
        RuntimeOrigin::signed(5),
        project_id,
        5
    ));

    let key = SumTreeName::ProjectTips {
        project_id: project_id,
        block_number: start_block_number,
    };

    let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
    assert_eq!(5, draws_in_round);

    let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
    assert_eq!(
        vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
        drawn_jurors
    );

    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    let period = SchellingGameShared::get_period(key.clone());

    assert_eq!(Some(Period::Commit), period);

    assert_ok!(ProjectTips::unstaking(RuntimeOrigin::signed(5), project_id));

    let hash = sp_io::hashing::keccak_256("0salt".as_bytes());
    assert_noop!(
        ProjectTips::commit_vote(RuntimeOrigin::signed(6), project_id, hash),
        <pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
    );
    let hash = sp_io::hashing::keccak_256("0salt".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(4),
        project_id,
        hash
    ));

    // You can replace vote within the commit period.
    let hash = sp_io::hashing::keccak_256("0salt2".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(4),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("0salt3".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(7),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("0salt4".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(13),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("0salt5".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(14),
        project_id,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt6".as_bytes());
    assert_ok!(ProjectTips::commit_vote(
        RuntimeOrigin::signed(15),
        project_id,
        hash
    ));

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length,
    );
    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_noop!(
        ProjectTips::reveal_vote(
            RuntimeOrigin::signed(4),
            project_id,
            2,
            "salt2".as_bytes().to_vec()
        ),
        <pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
    );

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(4),
        project_id,
        0,
        "salt2".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(7),
        project_id,
        0,
        "salt3".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(14),
        project_id,
        0,
        "salt5".as_bytes().to_vec()
    ));

    assert_ok!(ProjectTips::reveal_vote(
        RuntimeOrigin::signed(15),
        project_id,
        1,
        "salt6".as_bytes().to_vec()
    ));

    assert_noop!(
        ProjectTips::pass_period(RuntimeOrigin::signed(5), project_id),
        <pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length
            + phase_data.vote_length,
    );
    assert_ok!(ProjectTips::pass_period(
        RuntimeOrigin::signed(5),
        project_id
    ));

    assert_noop!(
        ProjectTips::add_incentive_count(RuntimeOrigin::signed(13), project_id),
        <pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
    );
    assert_ok!(ProjectTips::add_incentive_count(
        RuntimeOrigin::signed(14),
        project_id
    ));
    assert_ok!(ProjectTips::add_incentive_count(
        RuntimeOrigin::signed(15),
        project_id
    ));
}

#[test]
fn schelling_game_incentives_get_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        full_schelling_game_func(2, 1);
        System::set_block_number(1000);
        full_schelling_game_func(3, 1000);

        let incentive_count = ProjectTips::incentives_count(14).unwrap();

        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 2,
            winner: 2,
            loser: 0,
            total_stake: 14 * 100 + 14 * 100,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        assert_eq!(incentive_count, incentive_count_eq);
        // println!("{:?}", incentive_count);

        let incentive_count = ProjectTips::incentives_count(15).unwrap();

        // println!("{:?}", incentive_count);

        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 2,
            winner: 0,
            loser: 2,
            total_stake: 15 * 100 + 15 * 100,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        assert_eq!(incentive_count, incentive_count_eq);

        // draw twenty schelling game
        // use 14 and 15, increase both loser and winner count.

        assert_noop!(
            ProjectTips::get_incentives(RuntimeOrigin::signed(15)),
            Error::<Test>::NotReachedMinimumDecision
        );

        System::set_block_number(2000);

        full_schelling_game_func2(4, 2000);

        System::set_block_number(3000);

        full_schelling_game_func2(5, 3000);

        let incentive_count = ProjectTips::incentives_count(14).unwrap();

        // println!("incentive count:{:?}", incentive_count);

        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 4,
            winner: 2,
            loser: 2,
            total_stake: 14 * 100 + 14 * 100 + 14 * 100 + 14 * 100,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        assert_eq!(incentive_count, incentive_count_eq);

        let incentive_count = ProjectTips::incentives_count(15).unwrap();

        // println!("{:?}", incentive_count);

        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 4,
            winner: 2,
            loser: 2,
            total_stake: 15 * 100 + 15 * 100 + 15 * 100 + 15 * 100,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        assert_eq!(incentive_count, incentive_count_eq);
        for x in 4..20 {
            System::set_block_number(x * 1000);
            full_schelling_game_func(x, x * 1000);
        }

        let incentive_count = ProjectTips::incentives_count(14).unwrap();

        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 20,
            winner: 18,
            loser: 2,
            total_stake: 14 * 100 * 20,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        assert_eq!(incentive_count, incentive_count_eq);
        // println!("incentive count:{:?}", incentive_count);

        let balance = Balances::free_balance(14);

        // println!("balance account before(14):{:?}", balance);

        assert_ok!(ProjectTips::get_incentives(RuntimeOrigin::signed(14)));

        let balance = Balances::free_balance(14);

        // println!("balance account after(14):{:?}", balance);

        let incentive_count = ProjectTips::incentives_count(15).unwrap();

        let incentive_count_eq: Incentives<Test> = Incentives {
            number_of_games: 20,
            winner: 2,
            loser: 18,
            total_stake: 15 * 100 * 20,
            start: WhenDetails {
                block: 201,
                time: 0,
            },
        };

        assert_eq!(incentive_count, incentive_count_eq);
        // println!("incentive count:{:?}", incentive_count);

        let balance = Balances::free_balance(15);

        // println!("balance account before(15):{:?}", balance);

        assert_ok!(ProjectTips::get_incentives(RuntimeOrigin::signed(15)));

        let balance = Balances::free_balance(15);

        // println!("balance account after(15):{:?}", balance);
    })
}

#[test]
fn get_tip_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let balance_start = Balances::free_balance(35);

        // println!("balance account before:{:?}", balance_start);
        full_schelling_game_func(35, 1);

        let tipping_name = TippingName::SmallTipper;
        let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
        let max_tipping_value = tipping_value.max_tipping_value;
        // let stake_required = tipping_value.stake_required;
        let funding_needed = max_tipping_value - 100;

        let project_ids = ProjectTips::get_projects_from_accounts(35);
        let project_id = project_ids.last().unwrap();
        let project_id = *project_id;
        // println!("project id{}", project_id);

        // who_ask_tipper is same as who apply staking for staking period
        let _balance_after_stake = Balances::free_balance(35);

        // println!("balance account before:{:?}", balance_after_stake);

        assert_ok!(ProjectTips::release_tip(
            RuntimeOrigin::signed(14),
            project_id
        ));

        let balance = Balances::free_balance(35);

        // println!("balance account after:{:?}", balance);
        // println!("balance account after:{:?}", balance_start + funding_needed);

        assert_eq!(balance, balance_start + funding_needed);

        assert_noop!(
            ProjectTips::release_tip(RuntimeOrigin::signed(15), project_id),
            Error::<Test>::AlreadyFunded
        );

        System::set_block_number(1000);
        // account: 34
        let balance_start = Balances::free_balance(34);
        // println!("balance account before:{:?}", balance_start);

        full_schelling_game_func_ask_tipper_defeated(34, 1000);

        let project_ids = ProjectTips::get_projects_from_accounts(34);
        let project_id = project_ids.last().unwrap();
        let project_id = *project_id;
        // println!("project id{}", project_id);

        // who_ask_tipper is same as who apply staking for staking period
        let _balance_after_stake = Balances::free_balance(34);

        //   println!("balance account before:{:?}", balance_after_stake);

        assert_ok!(ProjectTips::release_tip(
            RuntimeOrigin::signed(14),
            project_id
        ));

        let balance = Balances::free_balance(34);

        //   println!("balance account after:{:?}", balance);
        //   println!("balance account after:{:?}", balance_start + funding_needed);

        assert_eq!(balance, balance_start);

        assert_noop!(
            ProjectTips::release_tip(RuntimeOrigin::signed(15), project_id),
            Error::<Test>::AlreadyFunded
        );
    })
}
