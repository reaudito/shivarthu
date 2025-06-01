use crate::types::BountyStatus;
use crate::{mock::*, Error, Event};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{assert_noop, assert_ok};
use sp_std::collections::btree_map::BTreeMap;

#[test]
fn add_candidate_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_eq!(TemplateModule::candidates_by_group(1), vec![2]);
        System::assert_last_event(Event::CandidateAdded { candidate: 2 }.into());
    });
}

#[test]
fn add_candidate_fails_if_already_exists() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));
        assert_noop!(
            TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1),
            Error::<Test>::CandidateExists
        );
    });
}

#[test]
fn vote_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(3), 1));

        let vote_map = BTreeMap::from([(2, 3u8), (3, 2u8)]);
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            vote_map.clone()
        ));

        assert_eq!(TemplateModule::votes_by_group(1, &4), vote_map);
        assert_eq!(
            TemplateModule::total_votes_by_group(1),
            BTreeMap::from([(2, 3u32), (3, 2u32)])
        );
        System::assert_last_event(Event::VoteCast { user: 4 }.into());
    });
}

#[test]
fn vote_fails_if_candidate_does_not_exist_in_group() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));
        let bad_vote = BTreeMap::from([(99, 4u8)]);
        assert_noop!(
            TemplateModule::vote(RuntimeOrigin::signed(4), 1, bad_vote),
            Error::<Test>::NoSuchCandidate
        );
    });
}

#[test]
fn get_top_n_winners_works() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(3), 1));

        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            BTreeMap::from([(2, 2u8), (3, 1u8)])
        ));
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(5),
            1,
            BTreeMap::from([(2, 1u8)])
        ));
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(6),
            1,
            BTreeMap::from([(2, 2u8)])
        ));

        let top_winners = TemplateModule::get_top_n_winners(1, 2);
        assert_eq!(top_winners, vec![(2, 5), (3, 1)]);
    });
}

#[test]
fn clean_up_votes_works() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        use pallet_timestamp as timestamp;

        System::set_block_number(1);
        timestamp::Pallet::<Test>::set_timestamp(0u64);

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            BTreeMap::from([(2, 3u8)])
        ));

        let winners = TemplateModule::get_top_n_winners(1, 3);
        assert_eq!(winners, vec![(2, 3)]);

        let expired_time = 1000 * 60 * 60 * 24 * 91;
        timestamp::Pallet::<Test>::set_timestamp(expired_time);

        TemplateModule::remove_stale_votes(1);
        assert_eq!(
            TemplateModule::votes_by_group(1, 4),
            BTreeMap::<u64, u8>::new()
        );
        assert_eq!(TemplateModule::vote_timestamps(1, 4), None);
        assert_eq!(TemplateModule::get_top_n_winners(1, 3), vec![]);
    });
}

#[test]
fn vote_allows_second_vote_and_updates_top_winners() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(3), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(8), 1));

        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            BTreeMap::from([(2, 2u8), (3, 1u8)])
        ));
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(5),
            1,
            BTreeMap::from([(2, 2u8)])
        ));

        let winners = TemplateModule::get_top_n_winners(1, 2);
        assert_eq!(winners, vec![(2, 4), (3, 1)]);

        // Voter 4 revotes with score for 8
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            BTreeMap::from([(8, 3u8)])
        ));

        let winners = TemplateModule::get_top_n_winners(1, 3);
        assert_eq!(winners, vec![(2, 2), (8, 3)]);
        assert_eq!(TemplateModule::total_votes_by_group(1).get(&3), None);
    });
}

#[test]
fn vote_with_zero_score_is_ignored() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));

        assert_noop!(
            TemplateModule::vote(RuntimeOrigin::signed(2), 1, BTreeMap::from([(1, 0u8)])),
            Error::<Test>::ScoreZeroOrLess
        );
    });
}

#[test]
fn vote_fails_if_score_exceeds_max() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));
        let invalid_vote = BTreeMap::from([(1, 255u8)]); // assuming max score is 10
        assert_noop!(
            TemplateModule::vote(RuntimeOrigin::signed(2), 1, invalid_vote),
            Error::<Test>::ScoreTooHigh
        );
    });
}

#[test]
fn votes_are_isolated_by_group() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 2));

        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(3),
            1,
            BTreeMap::from([(1, 3u8)])
        ));
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(3),
            2,
            BTreeMap::from([(2, 5u8)])
        ));

        assert_eq!(
            TemplateModule::total_votes_by_group(1),
            BTreeMap::from([(1, 3u32)])
        );
        assert_eq!(
            TemplateModule::total_votes_by_group(2),
            BTreeMap::from([(2, 5u32)])
        );
    });
}

#[test]
fn overwrite_vote_only_updates_user_contribution() {
    new_test_ext().execute_with(|| {
        initialize_mock_members();

        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));

        // Voter 4 votes
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            BTreeMap::from([(1, 2u8), (2, 3u8)])
        ));

        // Voter 5 votes
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(5),
            1,
            BTreeMap::from([(1, 5u8)])
        ));

        // Voter 4 revotes
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            BTreeMap::from([(2, 1u8)])
        ));

        // Result: (1 loses 2 from 4, but keeps 5 from 5), (2 goes from 3 to 1)
        assert_eq!(
            TemplateModule::total_votes_by_group(1),
            BTreeMap::from([(1, 5u32), (2, 1u32)])
        );
    });
}

#[test]
fn start_bounty_vote_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        let beneficiary = 1;
        let amount = 1000 as u64;

        // Start bounty vote
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            amount,
            1 // group_id
        ));

        // Check that bounty vote start time is set
        assert!(TemplateModule::bounty_vote_start(&beneficiary).is_some());

        // Check that bounty approval structure is initialized
        let approval = TemplateModule::bounty_approval(&beneficiary);
        assert_eq!(approval.approvals, 0);
        assert_eq!(approval.rejections, 0);

        // Check that bounty amount is stored
        assert_eq!(TemplateModule::bounty_amount(&beneficiary), Some(amount));

        // Check that bounty status is active
        assert_eq!(
            TemplateModule::bounty_user_status(&beneficiary),
            Some(BountyStatus::Active)
        );

        // Event should be emitted
        System::assert_last_event(
            Event::BountyVoteStarted {
                recipient: beneficiary,
            }
            .into(),
        );
    });
}

#[test]
fn vote_on_bounty_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        let beneficiary = 1;
        let voter = 2;
        let group_id = 1;

        // Setup: start a bounty vote
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            1000,
            group_id
        ));

        // Add some candidates and votes to simulate top winners
        assert_ok!(TemplateModule::add_candidate(
            RuntimeOrigin::signed(voter),
            group_id
        ));
        let mut scores = BTreeMap::new();
        scores.insert(voter.clone(), 5);
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(voter),
            group_id,
            scores
        ));

        // Now cast a bounty vote
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            beneficiary,
            true // approve
        ));

        // Check that the vote is recorded
        assert_eq!(
            TemplateModule::bounty_votes(&beneficiary, &voter),
            Some(true)
        );

        // Check approval counts
        let approval = TemplateModule::bounty_approval(&beneficiary);
        assert_eq!(approval.approvals, 1);
        assert_eq!(approval.rejections, 0);

        // Event should be emitted
        System::assert_last_event(Event::VoteCast { user: voter }.into());
    });
}

#[test]
fn finalize_bounty_release_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        let beneficiary = 2;
        let voter = 3;
        let group_id = 1;
        let amount = 1000u64;

        // Setup: start a bounty vote
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            amount,
            group_id
        ));

        // Add candidate and vote to get a winner
        assert_ok!(TemplateModule::add_candidate(
            RuntimeOrigin::signed(voter),
            group_id
        ));
        let mut scores = BTreeMap::new();
        scores.insert(voter.clone(), 5);
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(voter),
            group_id,
            scores
        ));

        // Cast a bounty vote
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            beneficiary,
            true
        ));

        // Fast forward time (simulate 31 days passed)
        let now = <pallet_timestamp::Pallet<Test>>::get();
        let one_month_plus = 1000 * 60 * 60 * 24 * 31_u64; // in ms
        <pallet_timestamp::Pallet<Test>>::set_timestamp(now + one_month_plus);

        // Finalize bounty release
        assert_ok!(TemplateModule::finalize_bounty_release(
            RuntimeOrigin::signed(4),
            beneficiary
        ));

        // Check event
        System::assert_last_event(
            Event::BountyReleased {
                recipient: beneficiary,
                amount,
            }
            .into(),
        );

        // Ensure storage cleanup
        assert_eq!(TemplateModule::bounty_vote_start(&beneficiary), None);
        assert_eq!(TemplateModule::bounty_user_status(&beneficiary), None);
        assert_eq!(TemplateModule::bounty_amount(&beneficiary), None);
        assert_eq!(TemplateModule::bounty_group(&beneficiary), None);
        let approval = TemplateModule::bounty_approval(&beneficiary);
        assert_eq!(approval.approvals, 0);
        assert_eq!(approval.rejections, 0);
        assert_eq!(TemplateModule::bounty_votes(&beneficiary, &voter), None);
    });
}

#[test]
fn finalize_bounty_release_fails_if_not_enough_time_passed() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        let beneficiary = 1;

        // Setup: start a bounty vote
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            1000,
            1
        ));

        // Try to finalize before 1 month
        assert_noop!(
            TemplateModule::finalize_bounty_release(RuntimeOrigin::signed(3), beneficiary),
            Error::<Test>::InvalidBountyState
        );
    });
}

#[test]
fn finalize_bounty_release_fails_without_majority() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        let beneficiary = 1;
        let voter = 2;
        let group_id = 1;

        // Setup: start a bounty vote
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            1000,
            group_id
        ));

        // Add candidate and vote to get a winner
        assert_ok!(TemplateModule::add_candidate(
            RuntimeOrigin::signed(voter),
            group_id
        ));
        let mut scores = BTreeMap::new();
        scores.insert(voter.clone(), 5);
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(voter),
            group_id,
            scores
        ));

        // Cast a reject vote
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            beneficiary,
            false
        ));

        // Fast forward time (simulate 31 days passed)
        let now = <pallet_timestamp::Pallet<Test>>::get();
        let one_month_plus = 1000 * 60 * 60 * 24 * 31_u64; // in ms
        <pallet_timestamp::Pallet<Test>>::set_timestamp(now + one_month_plus);

        // Finalize bounty release -> should fail due to lack of majority
        assert_noop!(
            TemplateModule::finalize_bounty_release(RuntimeOrigin::signed(3), beneficiary),
            Error::<Test>::NotSuperMajoriy
        );
    });
}

#[test]
fn test_two_bounties_for_same_beneficiary_success() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        let beneficiary = 2;
        let voter = 3;
        let group_id = 1;
        let amount = 1000u64;

        // --- First Bounty ---
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            amount,
            group_id
        ));

        // Add candidate and vote
        assert_ok!(TemplateModule::add_candidate(
            RuntimeOrigin::signed(voter),
            group_id
        ));
        let mut scores = BTreeMap::new();
        scores.insert(voter.clone(), 5);
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(voter),
            group_id,
            scores
        ));

        // Cast a bounty vote
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            beneficiary,
            true
        ));

        // Fast forward time
        // Fast forward time (simulate 31 days passed)
        let now = <pallet_timestamp::Pallet<Test>>::get();
        let one_month_plus = 1000 * 60 * 60 * 24 * 31_u64; // in ms
        <pallet_timestamp::Pallet<Test>>::set_timestamp(now + one_month_plus);

        // Finalize bounty release
        assert_ok!(TemplateModule::finalize_bounty_release(
            RuntimeOrigin::signed(4),
            beneficiary
        ));

        // Check event
        System::assert_last_event(
            Event::BountyReleased {
                recipient: beneficiary,
                amount,
            }
            .into(),
        );

        // --- Second Bounty ---
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            amount,
            group_id
        ));

        // Reuse the same voter and cast another vote
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            beneficiary,
            true
        ));

        // Fast forward again
        // Fast forward time (simulate 31 days passed)
        let now = <pallet_timestamp::Pallet<Test>>::get();
        let one_month_plus = 1000 * 60 * 60 * 24 * 31_u64; // in ms
        <pallet_timestamp::Pallet<Test>>::set_timestamp(now + one_month_plus);

        // Finalize second bounty
        assert_ok!(TemplateModule::finalize_bounty_release(
            RuntimeOrigin::signed(4),
            beneficiary
        ));

        // Check second event
        System::assert_last_event(
            Event::BountyReleased {
                recipient: beneficiary,
                amount,
            }
            .into(),
        );

        // Ensure all storage cleanup happened twice
        assert_eq!(TemplateModule::bounty_user_status(&beneficiary), None);
        assert_eq!(TemplateModule::bounty_vote_start(&beneficiary), None);
        assert_eq!(TemplateModule::bounty_amount(&beneficiary), None);
        assert_eq!(TemplateModule::bounty_group(&beneficiary), None);

        let approval = TemplateModule::bounty_approval(&beneficiary);
        assert_eq!(approval.approvals, 0);
        assert_eq!(approval.rejections, 0);
    });
}

#[test]
fn test_cannot_start_second_bounty_while_first_active() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();

        let beneficiary = 2;
        let group_id = 1;
        let amount = 1000u64;

        // Start first bounty
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(beneficiary),
            amount,
            group_id
        ));

        // Try to start second bounty while first is active
        assert_noop!(
            TemplateModule::start_bounty_vote(RuntimeOrigin::signed(beneficiary), amount, group_id),
            Error::<Test>::InvalidBountyState
        );
    });
}
