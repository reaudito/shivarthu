use crate::types::Incentives;
use crate::types::Post;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_schelling_game_shared::types::Period;
use pallet_sortition_sum_game::types::SumTreeName;
use pallet_support::WhenDetails;
use pallet_support::{Content, WhoAndWhen};

#[test]
fn test_positive_externality_post() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::create_positive_externality_post(
            RuntimeOrigin::signed(1),
            Content::None
        ));
        let post = TemplateModule::post_by_id(1);
        let post_compare = Some(Post {
            id: 1,
            created: WhoAndWhen {
                account: 1,
                block: 0,
                time: 0,
            },
            edited: false,
            owner: 1,
            content: Content::None,
            hidden: false,
            upvotes_count: 0,
            downvotes_count: 0,
        });
        assert_eq!(post, post_compare);
        //    assert_ok!(TemplateModule::apply_jurors(Origin::signed(1), 2, 60));
    });
}

#[test]
fn test_setting_positive_externality_validation() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::set_validate_positive_externality(
            RuntimeOrigin::signed(1),
            true
        ));
        let value = TemplateModule::validate(1);
        assert_eq!(value, true);
    });
}

#[test]
fn test_applying_for_staking_period() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::set_validate_positive_externality(
            RuntimeOrigin::signed(1),
            true
        ));
        System::set_block_number(1298000);
        assert_ok!(TemplateModule::apply_staking_period(
            RuntimeOrigin::signed(2),
            1
        ));
        System::set_block_number(1298000 + 1298000);
        assert_ok!(TemplateModule::apply_staking_period(
            RuntimeOrigin::signed(2),
            1
        ));
    });
}

#[test]
fn test_appying_jurors() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::set_validate_positive_externality(
            RuntimeOrigin::signed(1),
            true
        ));
        // System::set_block_number(1298000);
        assert_ok!(TemplateModule::apply_staking_period(
            RuntimeOrigin::signed(2),
            1
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(4),
            1,
            1000
        ));
    });
}

#[test]
fn test_change_period() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::set_validate_positive_externality(
            RuntimeOrigin::signed(1),
            true
        ));
        System::set_block_number(1298000);
        assert_ok!(TemplateModule::apply_staking_period(
            RuntimeOrigin::signed(2),
            1
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(4),
            1,
            1000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(5),
            1,
            2000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(6),
            1,
            3000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(7),
            1,
            4000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(8),
            1,
            5000
        ));
        System::set_block_number(1298080);
        assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
    })
}

#[test]
fn test_draw_jurors_period() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::set_validate_positive_externality(
            RuntimeOrigin::signed(1),
            true
        ));
        System::set_block_number(1298000);
        assert_ok!(TemplateModule::apply_staking_period(
            RuntimeOrigin::signed(2),
            1
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(4),
            1,
            1000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(5),
            1,
            2000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(6),
            1,
            3000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(7),
            1,
            4000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(8),
            1,
            5000
        ));
        System::set_block_number(1298080);
        assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
        assert_ok!(TemplateModule::draw_jurors(RuntimeOrigin::signed(8), 1, 5));
    })
}

#[test]
fn test_drawn_jurors() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::set_validate_positive_externality(
            RuntimeOrigin::signed(1),
            true
        ));
        System::set_block_number(1298000);
        assert_ok!(TemplateModule::apply_staking_period(
            RuntimeOrigin::signed(2),
            1
        ));
        let balance = Balances::free_balance(4);
        assert_eq!(300000, balance);
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(4),
            1,
            1000
        ));
        let balance = Balances::free_balance(4);
        assert_eq!(299000, balance);
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(5),
            1,
            2000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(6),
            1,
            3000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(7),
            1,
            4000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(8),
            1,
            5000
        ));
        System::set_block_number(1298080);
        assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
        assert_ok!(TemplateModule::draw_jurors(RuntimeOrigin::signed(8), 1, 5));
        let data = TemplateModule::get_drawn_jurors(1);
        assert_eq!(
            data,
            [(4, 1000), (5, 2000), (6, 3000), (7, 4000), (8, 5000)]
        );
        // println!("drawn jurors {:?}",data);
    })
}

#[test]
fn test_commit_and_incentives_vote() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::set_validate_positive_externality(
            RuntimeOrigin::signed(1),
            true
        ));
        System::set_block_number(1298000);
        assert_ok!(TemplateModule::apply_staking_period(
            RuntimeOrigin::signed(2),
            1
        ));
        let balance = Balances::free_balance(4);
        assert_eq!(300000, balance);
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(4),
            1,
            1000
        ));
        let balance = Balances::free_balance(4);
        assert_eq!(299000, balance);
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(5),
            1,
            2000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(6),
            1,
            3000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(7),
            1,
            4000
        ));
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(8),
            1,
            5000
        ));
        System::set_block_number(1298080);
        assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
        assert_ok!(TemplateModule::draw_jurors(RuntimeOrigin::signed(8), 1, 5));

        let data = TemplateModule::get_drawn_jurors(1);
        assert_eq!(
            data,
            [(4, 1000), (5, 2000), (6, 3000), (7, 4000), (8, 5000)]
        );
        assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));

        let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
        assert_ok!(TemplateModule::commit_vote(
            RuntimeOrigin::signed(4),
            1,
            hash
        ));

        let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
        assert_ok!(TemplateModule::commit_vote(
            RuntimeOrigin::signed(5),
            1,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("5salt3".as_bytes());
        assert_ok!(TemplateModule::commit_vote(
            RuntimeOrigin::signed(6),
            1,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
        assert_ok!(TemplateModule::commit_vote(
            RuntimeOrigin::signed(7),
            1,
            hash
        ));
        let hash = sp_io::hashing::keccak_256("5salt5".as_bytes());
        assert_ok!(TemplateModule::commit_vote(
            RuntimeOrigin::signed(8),
            1,
            hash
        ));
        System::set_block_number(12980160);
        assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
        assert_ok!(TemplateModule::reveal_vote(
            RuntimeOrigin::signed(4),
            1,
            1,
            "salt".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote(
            RuntimeOrigin::signed(5),
            1,
            1,
            "salt2".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote(
            RuntimeOrigin::signed(6),
            1,
            5,
            "salt3".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote(
            RuntimeOrigin::signed(7),
            1,
            1,
            "salt4".as_bytes().to_vec()
        ));
        assert_ok!(TemplateModule::reveal_vote(
            RuntimeOrigin::signed(8),
            1,
            5,
            "salt5".as_bytes().to_vec()
        ));
        System::set_block_number(12980260);
        assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));

        // assert_ok!(TemplateModule::get_incentives(RuntimeOrigin::signed(4), 1));
    })
}

// Play two schelling game to check incentives are updated

fn full_schelling_game_func(user_to_calculate: u64, start_block_number: u64) {
    assert_ok!(TemplateModule::set_validate_positive_externality(
        RuntimeOrigin::signed(1),
        true
    ));
    System::set_block_number(start_block_number);
    assert_ok!(TemplateModule::apply_staking_period(
        RuntimeOrigin::signed(2),
        user_to_calculate
    ));

    let phase_data = TemplateModule::get_phase_data();

    for j in 4..30 {
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(j),
            user_to_calculate,
            j * 100
        ));
    }

    assert_noop!(
        TemplateModule::draw_jurors(RuntimeOrigin::signed(5), user_to_calculate, 5),
        <pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
    );

    assert_noop!(
        TemplateModule::pass_period(RuntimeOrigin::signed(5), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
    );

    System::set_block_number(start_block_number + phase_data.staking_length);

    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    assert_ok!(TemplateModule::draw_jurors(
        RuntimeOrigin::signed(5),
        user_to_calculate,
        5
    ));

    let block_number = TemplateModule::validation_block(user_to_calculate);

    let key = SumTreeName::PositiveExternality {
        user_address: user_to_calculate,
        block_number: block_number,
    };

    let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
    assert_eq!(5, draws_in_round);

    let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
    assert_eq!(
        vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
        drawn_jurors
    );

    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    let period = SchellingGameShared::get_period(key.clone());

    assert_eq!(Some(Period::Commit), period);

    assert_ok!(TemplateModule::unstaking(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_noop!(
        TemplateModule::commit_vote(RuntimeOrigin::signed(6), user_to_calculate, hash),
        <pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
    );
    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(4),
        user_to_calculate,
        hash
    ));

    // You can replace vote within the commit period.
    let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(4),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(7),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(13),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(14),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("3salt6".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(15),
        user_to_calculate,
        hash
    ));

    assert_noop!(
        TemplateModule::pass_period(RuntimeOrigin::signed(5), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length,
    );
    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    assert_noop!(
        TemplateModule::reveal_vote(
            RuntimeOrigin::signed(4),
            user_to_calculate,
            2,
            "salt2".as_bytes().to_vec()
        ),
        <pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
    );

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(4),
        user_to_calculate,
        1,
        "salt2".as_bytes().to_vec()
    ));

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(7),
        user_to_calculate,
        1,
        "salt3".as_bytes().to_vec()
    ));

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(14),
        user_to_calculate,
        1,
        "salt5".as_bytes().to_vec()
    ));

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(15),
        user_to_calculate,
        3,
        "salt6".as_bytes().to_vec()
    ));

    assert_noop!(
        TemplateModule::pass_period(RuntimeOrigin::signed(5), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length
            + phase_data.vote_length,
    );
    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    assert_ok!(TemplateModule::set_new_mean_value(
        RuntimeOrigin::signed(13),
        user_to_calculate
    ));

    assert_noop!(
        TemplateModule::add_incentive_count(RuntimeOrigin::signed(13), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
    );
    assert_ok!(TemplateModule::add_incentive_count(
        RuntimeOrigin::signed(14),
        user_to_calculate
    ));
    assert_ok!(TemplateModule::add_incentive_count(
        RuntimeOrigin::signed(15),
        user_to_calculate
    ));
}

fn full_schelling_game_func2(user_to_calculate: u64, start_block_number: u64) {

    assert_ok!(TemplateModule::set_validate_positive_externality(
        RuntimeOrigin::signed(1),
        true
    ));
    System::set_block_number(start_block_number);
    assert_ok!(TemplateModule::apply_staking_period(
        RuntimeOrigin::signed(2),
        user_to_calculate
    ));

    let phase_data = TemplateModule::get_phase_data();

    for j in 4..30 {
        assert_ok!(TemplateModule::apply_jurors(
            RuntimeOrigin::signed(j),
            user_to_calculate,
            j * 100
        ));
    }

    assert_noop!(
        TemplateModule::draw_jurors(RuntimeOrigin::signed(5), user_to_calculate, 5),
        <pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
    );

    assert_noop!(
        TemplateModule::pass_period(RuntimeOrigin::signed(5), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
    );

    System::set_block_number(start_block_number + phase_data.staking_length);

    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    assert_ok!(TemplateModule::draw_jurors(
        RuntimeOrigin::signed(5),
        user_to_calculate,
        5
    ));
    let block_number = TemplateModule::validation_block(user_to_calculate);

    let key = SumTreeName::PositiveExternality {
        user_address: user_to_calculate,
        block_number: block_number,
    };

    let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
    // println!("Draws in round{draws_in_round}");
    assert_eq!(5, draws_in_round);

    let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
    assert_eq!(
        vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)],
        drawn_jurors
    );

    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    let period = SchellingGameShared::get_period(key.clone());

    assert_eq!(Some(Period::Commit), period);

    assert_ok!(TemplateModule::unstaking(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_noop!(
        TemplateModule::commit_vote(RuntimeOrigin::signed(6), user_to_calculate, hash),
        <pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
    );
    let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(4),
        user_to_calculate,
        hash
    ));

    // You can replace vote within the commit period.
    let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(4),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(7),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(13),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("3salt5".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(14),
        user_to_calculate,
        hash
    ));

    let hash = sp_io::hashing::keccak_256("1salt6".as_bytes());
    assert_ok!(TemplateModule::commit_vote(
        RuntimeOrigin::signed(15),
        user_to_calculate,
        hash
    ));

    assert_noop!(
        TemplateModule::pass_period(RuntimeOrigin::signed(5), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length,
    );
    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    assert_noop!(
        TemplateModule::reveal_vote(
            RuntimeOrigin::signed(4),
            user_to_calculate,
            2,
            "salt2".as_bytes().to_vec()
        ),
        <pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
    );

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(4),
        user_to_calculate,
        1,
        "salt2".as_bytes().to_vec()
    ));

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(7),
        user_to_calculate,
        1,
        "salt3".as_bytes().to_vec()
    ));

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(14),
        user_to_calculate,
        3,
        "salt5".as_bytes().to_vec()
    ));

    assert_ok!(TemplateModule::reveal_vote(
        RuntimeOrigin::signed(15),
        user_to_calculate,
        1,
        "salt6".as_bytes().to_vec()
    ));

    assert_noop!(
        TemplateModule::pass_period(RuntimeOrigin::signed(5), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
    );
    System::set_block_number(
        phase_data.evidence_length
            + start_block_number
            + phase_data.staking_length
            + phase_data.commit_length
            + phase_data.vote_length,
    );
    assert_ok!(TemplateModule::pass_period(
        RuntimeOrigin::signed(5),
        user_to_calculate
    ));

    assert_ok!(TemplateModule::set_new_mean_value(
        RuntimeOrigin::signed(13),
        user_to_calculate
    ));

    assert_noop!(
        TemplateModule::add_incentive_count(RuntimeOrigin::signed(13), user_to_calculate),
        <pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
    );
    assert_ok!(TemplateModule::add_incentive_count(
        RuntimeOrigin::signed(14),
        user_to_calculate
    ));
    assert_ok!(TemplateModule::add_incentive_count(
        RuntimeOrigin::signed(15),
        user_to_calculate
    ));
}

#[test]
fn schelling_game_incentives_get_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let startblock1 = 1 * (3 * 30 * 24 * 60 * 60) / 6;

        let startblock2 = 2 * (3 * 30 * 24 * 60 * 60) / 6;

        let startblock3 = 3 * (3 * 30 * 24 * 60 * 60) / 6;

        let startblock4 = 4 * (3 * 30 * 24 * 60 * 60) / 6;



        full_schelling_game_func(2, 1);

        let balance = Balances::free_balance(2);
        println!("{}", balance);
        // assert_eq!(300025, balance);
        assert_ok!(TemplateModule::release_positive_externality_fund(
            RuntimeOrigin::signed(14),
            2
        ));

        let balance = Balances::free_balance(2);
        println!("{}", balance);


        assert_noop!(
            TemplateModule::release_positive_externality_fund(RuntimeOrigin::signed(13), 2),
            Error::<Test>::AlreadyFunded
        );
        
        full_schelling_game_func(2, startblock1);

        let incentive_count = TemplateModule::incentives_count(14).unwrap();

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

        let incentive_count = TemplateModule::incentives_count(15).unwrap();

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
            TemplateModule::get_incentives(RuntimeOrigin::signed(15)),
            Error::<Test>::NotReachedMinimumDecision
        );


        full_schelling_game_func2(4, startblock2);


        full_schelling_game_func2(5, startblock3);

        let incentive_count = TemplateModule::incentives_count(14).unwrap();

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

        let incentive_count = TemplateModule::incentives_count(15).unwrap();

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
            System::set_block_number(x * startblock4);
            full_schelling_game_func(x, x * startblock4);
        }

        let incentive_count = TemplateModule::incentives_count(14).unwrap();

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

        assert_ok!(TemplateModule::get_incentives(RuntimeOrigin::signed(14)));

        let balance = Balances::free_balance(14);

        // println!("balance account after(14):{:?}", balance);

        let incentive_count = TemplateModule::incentives_count(15).unwrap();

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

        assert_ok!(TemplateModule::get_incentives(RuntimeOrigin::signed(15)));

        let balance = Balances::free_balance(15);

        // println!("balance account after(15):{:?}", balance);
    })
}
