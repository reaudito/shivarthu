use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_std::collections::btree_map::BTreeMap;

#[test]
fn add_candidate_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 2));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::candidates(), vec![2]);
		// Assert that the correct event was deposited
		System::assert_last_event(Event::CandidateAdded { candidate: 2 }.into());
	});
}

#[test]
fn add_candidate_fails_if_already_exists() {
	new_test_ext().execute_with(|| {
		// Add a candidate successfully
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 2));
		// Try to add the same candidate again
		assert_noop!(
			TemplateModule::add_candidate(RuntimeOrigin::signed(1), 2),
			Error::<Test>::CandidateExists
		);
	});
}

#[test]
fn vote_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Add candidates
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 2));
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 3));

		// Cast a vote
		assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(4), vec![2, 3]));

		// Debugging: Print all events
		// for event in System::events() {
		// 	println!("{:?}", event.event);
		// }

		// Check if votes are recorded in storage
		assert_eq!(TemplateModule::votes(&4), vec![2, 3]);
		assert_eq!(TemplateModule::total_votes(), BTreeMap::from([(2, 1), (3, 1)]));
		// Check if the event is emitted
		System::assert_last_event(Event::VoteCast { user: 4 }.into());
	});
}

#[test]
fn vote_fails_if_user_already_voted() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Add candidates
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 2));
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 3));

		// Cast a vote
		assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(4), vec![2, 3]));
		// Try to vote again
		assert_noop!(
			TemplateModule::vote(RuntimeOrigin::signed(4), vec![2]),
			Error::<Test>::AlreadyVoted
		);
	});
}

#[test]
fn vote_fails_if_candidate_does_not_exist() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Add candidates
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 3));
		// Try to vote for a non-existent candidate
		assert_noop!(
			TemplateModule::vote(RuntimeOrigin::signed(4), vec![2]),
			Error::<Test>::NoSuchCandidate
		);
	});
}

#[test]
fn get_top_n_winners_works() {
	new_test_ext().execute_with(|| {
		// Add candidates
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 2));
		assert_ok!(TemplateModule::add_candidate(RuntimeOrigin::signed(1), 3));

		// Cast votes
		assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(4), vec![2, 3]));
		assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(5), vec![2]));
		assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(6), vec![2]));
		assert_ok!(TemplateModule::vote(RuntimeOrigin::signed(7), vec![2]));

		// Get top 1 winner
		let top_winners = TemplateModule::get_top_n_winners(1);
		assert_eq!(top_winners, vec![(2, 4)]);

		// Get top 2 winners
		let top_winners = TemplateModule::get_top_n_winners(2);
		assert_eq!(top_winners, vec![(2, 4), (3, 1)]);
	});
}
