use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn simple_candidate_submission_should_work() {
	new_test_ext().execute_with(|| {
		let departmentid = 1;
		assert_eq!(candidate_ids(departmentid), Vec::<u64>::new());
		assert!(Elections::is_candidate(&1, departmentid).is_err());
		assert!(Elections::is_candidate(&2, departmentid).is_err());
		assert_eq!(balances(&1), (100000, 0));
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(1), departmentid));
		assert_eq!(balances(&1), (99997, 3));
		assert_eq!(candidate_ids(departmentid), vec![1]);

		assert!(Elections::is_candidate(&1, departmentid).is_ok());
		assert!(Elections::is_candidate(&2, departmentid).is_err());

		assert_eq!(balances(&2), (200000, 0));
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(2), departmentid));
		assert_eq!(balances(&2), (199997, 3));

		assert_eq!(candidate_ids(departmentid), vec![1, 2]);

		assert!(Elections::is_candidate(&1, departmentid).is_ok());
		assert!(Elections::is_candidate(&2, departmentid).is_ok());

		assert_eq!(candidate_deposit(&1, departmentid), 3);
		assert_eq!(candidate_deposit(&2, departmentid), 3);
		assert_eq!(candidate_deposit(&3, departmentid), 0);
	});
}

#[test]
fn candidates_are_always_sorted() {
	new_test_ext().execute_with(|| {
		let departmentid = 1;
		assert_eq!(candidate_ids(departmentid), Vec::<u64>::new());
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(3), departmentid));
		assert_eq!(candidate_ids(departmentid), vec![3]);
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(1), departmentid));
		assert_eq!(candidate_ids(departmentid), vec![1, 3]);
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(2), departmentid));
		assert_eq!(candidate_ids(departmentid), vec![1, 2, 3]);
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(4), departmentid));
		assert_eq!(candidate_ids(departmentid), vec![1, 2, 3, 4]);
	});
}

#[test]
fn simple_voting_should_work() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let departmentid = 1;
		assert_eq!(candidate_ids(departmentid), Vec::<u64>::new());
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(5), departmentid));
		assert_ok!(vote(RuntimeOrigin::signed(2), departmentid, vec![5], 20));
	});
}

#[test]
fn runners_up_should_be_kept() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let departmentid = 1;
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(5), departmentid));
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(4), departmentid));
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(3), departmentid));
		assert_ok!(submit_candidacy(RuntimeOrigin::signed(2), departmentid));

		assert_ok!(vote(RuntimeOrigin::signed(2), departmentid, vec![3], 20));
		assert_ok!(vote(RuntimeOrigin::signed(3), departmentid, vec![2], 30));
		assert_ok!(vote(RuntimeOrigin::signed(4), departmentid, vec![5], 40));
		assert_ok!(vote(RuntimeOrigin::signed(5), departmentid, vec![4], 50));

		assert_ok!(Elections::do_phragmen(RuntimeOrigin::signed(2), departmentid));
		// sorted based on account id.
		assert_eq!(members_ids(departmentid), vec![4, 5]);
		// sorted based on merit (least -> most)
		assert_eq!(runners_up_ids(departmentid), vec![3, 2]);
	});
}
