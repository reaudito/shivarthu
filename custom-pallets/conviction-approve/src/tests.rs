use super::*;
use crate::{mock::*, Error, Event, Something};
use frame_support::traits::Currency;
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);
        // Dispatch a signed extrinsic.
        assert_ok!(Template::do_something(RuntimeOrigin::signed(1), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(Something::<Test>::get(), Some(42));
        // Assert that the correct event was deposited
        System::assert_last_event(
            Event::SomethingStored {
                something: 42,
                who: 1,
            }
            .into(),
        );
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            Template::cause_error(RuntimeOrigin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}

#[test]
fn user_can_vote_and_tally_updates() {
    new_test_ext().execute_with(|| {
        let voter = 1;
        let group_id = 42;
        let balance = 200;
        let conviction = Conviction::Locked2x;

        // Vote aye
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(voter),
            group_id,
            true,
            conviction,
            balance,
        ));

        // Check vote recorded
        let votes = Template::group_votes(group_id);
        let record = votes.get(&voter).expect("vote should be stored");
        assert_eq!(record.vote.aye, true);
        assert_eq!(record.vote.balance, balance);
        assert_eq!(record.vote.conviction, conviction);

        // Check tally updated
        let (ayes, nays) = Template::vote_tally(group_id);
        let expected_weight = conviction.votes(balance).votes;
        assert_eq!(ayes, expected_weight);
        assert_eq!(nays, 0);

        // // Check lock applied
        // let locks = Balances::locks(voter);
        // assert!(locks.iter().any(|l| l.amount >= balance));
    });
}

#[test]
fn revote_should_update_tally_correctly() {
    new_test_ext().execute_with(|| {
        let voter = 1;
        let group_id = 100;
        let conviction1 = Conviction::Locked1x;
        let conviction2 = Conviction::Locked4x;
        let bal1 = 100;
        let bal2 = 300;

        // Initial vote (aye)
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(voter),
            group_id,
            true,
            conviction1,
            bal1
        ));

        let (ayes1, _) = Template::vote_tally(group_id);
        let expected1 = conviction1.votes(bal1).votes;
        assert_eq!(ayes1, expected1);

        // Revote (nay)
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(voter),
            group_id,
            false,
            conviction2,
            bal2
        ));

        let (ayes2, nays2) = Template::vote_tally(group_id);
        let expected2 = conviction2.votes(bal2).votes;
        assert_eq!(ayes2, 0); // old aye vote removed
        assert_eq!(nays2, expected2); // new nay vote added
    });
}
