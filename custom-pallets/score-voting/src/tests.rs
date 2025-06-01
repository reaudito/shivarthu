use crate::types::BountyStatus;
use crate::{mock::*, Error, Event};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::{assert_noop, assert_ok};
use pallet_support::Content;
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
        let recipient = 1;
        let group_id = 1;
        let content = Content::IPFS(b"QmSomeHash".to_vec());

        let bounty_id = TemplateModule::next_bounty_number();

        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(recipient),
            Some(1000),
            group_id,
            content.clone()
        ));

        let info = TemplateModule::bounty_info(bounty_id).unwrap();

        assert_eq!(info.recipient, recipient);
        assert_eq!(info.amount, Some(1000));
        assert_eq!(info.group_id, group_id);
        assert_eq!(info.status, BountyStatus::Active);
        assert_eq!(info.content, content);

        System::assert_last_event(
            Event::BountyVoteStarted {
                recipient,
                bounty_id,
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
        let recipient = 1;
        let voter = 2;
        let group_id = 1;
        let content = Content::Other(vec![1, 2, 3]);

        // Start bounty
        let bounty_id = TemplateModule::next_bounty_number();
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(recipient),
            Some(1000),
            group_id,
            content
        ));

        // Add candidates and votes to simulate top winners
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

        // Vote on bounty
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            bounty_id,
            true
        ));

        let info = TemplateModule::bounty_info(bounty_id).unwrap();
        assert_eq!(info.approval.approvals, 1);
        assert_eq!(info.approval.rejections, 0);
        assert_eq!(TemplateModule::bounty_votes(bounty_id, &voter), Some(true));

        System::assert_last_event(
            Event::BountyVoteCast {
                voter,
                bounty_id,
                approve: true,
            }
            .into(),
        );
    });
}

#[test]
fn finalize_bounty_release_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();
        let recipient = 1;
        let voter = 2;
        let group_id = 1;
        let content = Content::IPFS(b"QmHash".to_vec());

        // Start bounty
        let bounty_id = TemplateModule::next_bounty_number();
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(recipient),
            Some(1000),
            group_id,
            content
        ));

        // Setup voters
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

        // Vote
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            bounty_id,
            true
        ));

        // Fast forward time (simulate 31 days passed)
        let now = <pallet_timestamp::Pallet<Test>>::get();
        let one_month_plus = 1000 * 60 * 60 * 24 * 31_u64; // in ms
        <pallet_timestamp::Pallet<Test>>::set_timestamp(now + one_month_plus);
        // Finalize
        assert_ok!(TemplateModule::finalize_bounty_release(
            RuntimeOrigin::signed(3),
            bounty_id
        ));

        let info = TemplateModule::bounty_info(bounty_id).unwrap();
        assert_eq!(info.status, BountyStatus::Finalized);

        System::assert_last_event(
            Event::BountyReleased {
                recipient,
                amount: 1000,
            }
            .into(),
        );
    });
}

#[test]
fn finalize_bounty_release_fails_if_not_enough_time_passed() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();
        let recipient = 1;
        let group_id = 1;
        let content = Content::None;

        let bounty_id = TemplateModule::next_bounty_number();
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(recipient),
            Some(1000),
            group_id,
            content
        ));

        assert_noop!(
            TemplateModule::finalize_bounty_release(RuntimeOrigin::signed(2), bounty_id),
            Error::<Test>::InvalidBountyState
        );
    });
}

#[test]
fn cannot_vote_twice_on_same_bounty() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        initialize_mock_members();
        let recipient = 1;
        let voter = 2;
        let group_id = 1;
        let content = Content::Other(vec![1]);

        let bounty_id = TemplateModule::next_bounty_number();
        assert_ok!(TemplateModule::start_bounty_vote(
            RuntimeOrigin::signed(recipient),
            Some(1000),
            group_id,
            content
        ));

        // Setup winner
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

        // First vote
        assert_ok!(TemplateModule::vote_on_bounty(
            RuntimeOrigin::signed(voter),
            bounty_id,
            true
        ));

        // Second vote should fail
        assert_noop!(
            TemplateModule::vote_on_bounty(RuntimeOrigin::signed(voter), bounty_id, false),
            Error::<Test>::AlreadyVotedOnBounty
        );
    });
}
