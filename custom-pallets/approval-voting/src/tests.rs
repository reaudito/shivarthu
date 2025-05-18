use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_std::collections::btree_map::BTreeMap;

#[test]
fn add_candidate_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Add candidate 2 to group 1
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));

        // Check if candidate was added correctly for group 1
        assert_eq!(TemplateModule::candidates_by_group(1), vec![2]);

        // Check event
        System::assert_last_event(Event::CandidateAdded { candidate: 2 }.into());
    });
}

#[test]
fn add_candidate_fails_if_already_exists() {
    new_test_ext().execute_with(|| {
        // Add candidate 2 to group 1
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));
        // Try again
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

        // Add candidates to group 1
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(3), 1));

        // Cast a vote in group 1
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            vec![2, 3]
        ));

        // Check votes are recorded
        assert_eq!(TemplateModule::votes_by_group(1, &4), vec![2, 3]);

        // Check total votes
        assert_eq!(
            TemplateModule::total_votes_by_group(1),
            BTreeMap::from([(2, 1), (3, 1)])
        );

        // Check event
        System::assert_last_event(Event::VoteCast { user: 4 }.into());
    });
}

#[test]
fn vote_fails_if_candidate_does_not_exist_in_group() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Add one candidate
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 1));

        // Try voting for non-existent candidate
        assert_noop!(
            TemplateModule::vote(RuntimeOrigin::signed(4), 1, vec![99]),
            Error::<Test>::NoSuchCandidate
        );
    });
}

#[test]
fn get_top_n_winners_works() {
    new_test_ext().execute_with(|| {
        // Add candidates to group 1
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(3), 1));

        // Cast multiple votes in group 1
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            vec![2, 3]
        ));
        assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(5), 1, vec![2]));
        assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(6), 1, vec![2]));
        assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(7), 1, vec![2]));

        // Get top 1 winner
        let top_winners = TemplateModule::get_top_n_winners(1, 1);
        assert_eq!(top_winners, vec![(2, 4)]);

        // Get top 2 winners
        let top_winners = TemplateModule::get_top_n_winners(1, 2);
        assert_eq!(top_winners, vec![(2, 4), (3, 1)]);
    });
}

#[test]
fn clean_up_votes_works() {
    new_test_ext().execute_with(|| {
        use pallet_timestamp as timestamp;

        // Setup
        System::set_block_number(1);
        let now = 0u64;
        timestamp::Pallet::<Test>::set_timestamp(now);

        // Add candidates and vote
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(4), 1, vec![2]));
        let top_winners = TemplateModule::get_top_n_winners(1, 3);
        assert_eq!(top_winners, vec![(2, 1)]);

        // Advance time > 3 months (in ms)
        let three_months_later = 1000 * 60 * 60 * 24 * 91; // ~91 days
        timestamp::Pallet::<Test>::set_timestamp(three_months_later);
        assert_eq!(TemplateModule::votes_by_group(1, 4), vec![2]);

        // println!("{:?}", TemplateModule::votes_by_group(1, 4));
        // Run cleanup
        TemplateModule::remove_stale_votes(1);

        // Ensure vote is removed
        assert_eq!(TemplateModule::votes_by_group(1, 4), Vec::<u64>::new());
        assert_eq!(TemplateModule::vote_timestamps(1, 4), None);
        let top_winners = TemplateModule::get_top_n_winners(1, 3);
        assert_eq!(top_winners, vec![]);
    });
}

#[test]
fn vote_allows_second_vote_and_updates_top_winners() {
    new_test_ext().execute_with(|| {
        // Add candidates to group 1
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(2), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(3), 1));
        assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(8), 1));

        // First votes
        assert_ok!(TemplateModule::vote(
            RuntimeOrigin::signed(4),
            1,
            vec![2, 3]
        ));
        assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(5), 1, vec![2]));
        assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(6), 1, vec![2]));

        // Check initial top winners
        let top_winners = TemplateModule::get_top_n_winners(1, 2);
        assert_eq!(top_winners, vec![(2, 3), (3, 1)]);

        // Now voter 4 votes *again* and changes vote from [2,3] to just [8]
        assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(4), 1, vec![8]));

        // After second vote, top winners should reflect:
        // Candidate 2 loses 1 vote (from voter 4)
        // Candidate 3 loses 1 vote (from voter 4)
        // Candidate 8 gains 1 vote
        let top_winners = TemplateModule::get_top_n_winners(1, 3);
        assert_eq!(top_winners, vec![(2, 2), (8, 1)]);

        // Candidate 3 should have 0 votes now
        assert_eq!(TemplateModule::total_votes_by_group(1).get(&3), None);
    });
}
