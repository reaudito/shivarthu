use super::*;
use crate::types::{FundingInfo, FundingStatus, SpenderCategory};
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

pub const UNITS: u64 = 1_000_000_000_000;

#[test]
fn propose_funding_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS; // e.g., 10k UNITS
        let category = SpenderCategory::MediumSpender;

        // Min stake for MediumSpender is 50_000 * UNITS
        let min_stake = Template::min_stake(&category);
        assert_eq!(min_stake, 50 * UNITS);

        // Ensure proposer has enough balance
        let _ = Balances::deposit_creating(&proposer, min_stake);
        let funding_count_before = Template::next_funding_number();

        let content = Content::IPFS(b"QmSomeHash".to_vec());

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            content, // dummy content
            category.clone(),
        ));

        let funding_id = funding_count_before;

        // Check storage updated
        let info = Template::funding_info(funding_id).expect("funding should exist");
        assert_eq!(info.group_id, group_id);
        assert_eq!(info.amount, Some(amount));
        assert_eq!(info.status, FundingStatus::Active);
        assert_eq!(info.stake_amount, min_stake);

        // Stake should be reserved
        assert_eq!(Balances::reserved_balance(&proposer), min_stake);

        // Event emitted
        System::assert_last_event(
            Event::FundingProposed {
                proposer,
                funding_id,
                group_id,
                amount,
                stake: min_stake,
                category,
            }
            .into(),
        );
    });
}

#[test]
fn user_can_vote_and_tally_updates() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup: propose funding first
        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;
        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));
        let funding_id = 0;

        // Vote
        let voter = 2;
        let conviction = Conviction::Locked2x;
        let balance = 200;

        assert_ok!(Template::vote(
            RuntimeOrigin::signed(voter),
            funding_id,
            true,
            conviction,
            balance,
        ));

        // Check vote recorded
        let votes = Template::funding_votes(funding_id);
        let record = votes.get(&voter).expect("vote should be stored");
        assert_eq!(record.vote.aye, true);
        assert_eq!(record.vote.balance, balance);
        assert_eq!(record.vote.conviction, conviction);

        // Check tally
        let (ayes, nays) = Template::funding_tally(funding_id);
        let expected_weight = conviction.votes(balance).votes;
        assert_eq!(ayes, expected_weight);
        assert_eq!(nays, 0);

        // Check lock applied
        let locks = Balances::locks(&voter);
        assert!(locks.iter().any(|l| l.amount >= balance));
    });
}

#[test]
fn revote_should_update_tally_correctly() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup: propose funding
        let proposer = 1;
        let group_id = 100;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;
        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));
        let funding_id = 0;

        let voter = 2;
        let conviction1 = Conviction::Locked1x;
        let conviction2 = Conviction::Locked4x;
        let bal1 = 100;
        let bal2 = 300;

        // Initial vote (aye)
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(voter),
            funding_id,
            true,
            conviction1,
            bal1
        ));

        let (ayes1, _) = Template::funding_tally(funding_id);
        let expected1 = conviction1.votes(bal1).votes;
        assert_eq!(ayes1, expected1);

        // Revote (nay)
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(voter),
            funding_id,
            false,
            conviction2,
            bal2
        ));

        let (ayes2, nays2) = Template::funding_tally(funding_id);
        let expected2 = conviction2.votes(bal2).votes;
        assert_eq!(ayes2, 0); // old aye vote removed
        assert_eq!(nays2, expected2); // new nay vote added
    });
}

#[test]
fn multiple_voters_tally_correctly() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup: propose funding
        let proposer = 1;
        let group_id = 77;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;
        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));
        let funding_id = 0;

        // Voters and their votes
        let votes = vec![
            (2, true, Conviction::Locked1x, 100),  // aye, 1x
            (3, true, Conviction::Locked2x, 200),  // aye, 2x
            (4, false, Conviction::Locked3x, 150), // nay, 3x
        ];

        for (voter, aye, conviction, balance) in &votes {
            assert_ok!(Template::vote(
                RuntimeOrigin::signed(*voter),
                funding_id,
                *aye,
                *conviction,
                *balance,
            ));
        }

        // Fetch final tallies
        let (ayes, nays) = Template::funding_tally(funding_id);

        // Manually compute expected tallies
        let expected_ayes =
            Conviction::Locked1x.votes(100).votes + Conviction::Locked2x.votes(200).votes;
        let expected_nays = Conviction::Locked3x.votes(150).votes;

        assert_eq!(ayes, expected_ayes);
        assert_eq!(nays, expected_nays);

        // Check all votes are stored
        for (voter, aye, conviction, balance) in &votes {
            let group_votes = Template::funding_votes(funding_id);
            let stored = group_votes.get(voter).expect("vote should exist");

            assert_eq!(stored.vote.aye, *aye);
            assert_eq!(stored.vote.conviction, *conviction);
            assert_eq!(stored.vote.balance, *balance);

            let locks = Balances::locks(&voter);
            assert!(locks.iter().any(|l| l.amount >= *balance));
        }
    });
}

#[test]
fn unlock_works_after_expiry() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup: propose funding
        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;
        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));
        let funding_id = 0;

        let conviction = Conviction::Locked1x;
        let balance = 100;
        let voters = vec![2, 3, 4];

        // Step 1: Vote with lock
        for voter in &voters {
            assert_ok!(Template::vote(
                RuntimeOrigin::signed(*voter),
                funding_id,
                true,
                conviction,
                balance,
            ));

            // Ensure lock is applied
            let locks = Balances::locks(&voter);
            assert!(locks.iter().any(|l| l.amount >= balance));
        }

        // Step 2: Advance block past expiry
        let lock_blocks = conviction.lock_periods();
        System::set_block_number(
            System::block_number() + Template::u64_to_block_saturated((lock_blocks + 1).into()),
        );

        // Step 3: Unlock
        for voter in &voters {
            assert_ok!(Template::unlock(RuntimeOrigin::signed(*voter), funding_id));

            // Ensure lock is gone
            let locks = Balances::locks(&voter);
            assert!(locks.iter().all(|l| l.amount == 0));

            // Ensure vote is removed
            let votes = Template::funding_votes(funding_id);
            assert!(!votes.contains_key(voter));
        }
    });
}

#[test]
fn unlock_fails_if_not_expired() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup: propose funding
        let proposer = 1;
        let group_id = 99;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;
        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));
        let funding_id = 0;

        let voter = 2;
        let conviction = Conviction::Locked2x;
        let balance = 50;

        assert_ok!(Template::vote(
            RuntimeOrigin::signed(voter),
            funding_id,
            false,
            conviction,
            balance
        ));

        // Try to unlock too early
        assert_noop!(
            Template::unlock(RuntimeOrigin::signed(voter), funding_id),
            Error::<Test>::VoteStillLocked
        );
    });
}

#[test]
fn finalize_vote_emits_event_after_expiry() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Propose funding
        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;

        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));

        let funding_id = 0;

        // Advance block past expiry
        let duration = Template::u64_to_block_saturated(30 * 24 * 60 * 60 / 6);
        System::set_block_number(System::block_number() + duration + 1);

        // Finalize vote
        assert_ok!(Template::finalize_vote(
            RuntimeOrigin::signed(2),
            funding_id
        ));

        // Check event emitted
        System::assert_last_event(
            Event::FundingFinalized {
                funding_id,
                group_id,
                approved: false,
                total_ayes: 0,
                total_nays: 0,
            }
            .into(),
        );
    });
}

#[test]
fn finalize_vote_approved_when_ayes_majority() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;

        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));

        let funding_id = 0;

        // Aye votes
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(2),
            funding_id,
            true,
            Conviction::Locked3x,
            300,
        ));
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(3),
            funding_id,
            true,
            Conviction::Locked2x,
            200,
        ));

        // Nay vote
        assert_ok!(Template::vote(
            RuntimeOrigin::signed(4),
            funding_id,
            false,
            Conviction::Locked1x,
            100,
        ));

        let duration = Template::u64_to_block_saturated(30 * 24 * 60 * 60 / 6);
        System::set_block_number(System::block_number() + duration + 1);

        assert_ok!(Template::finalize_vote(
            RuntimeOrigin::signed(5),
            funding_id
        ));

        // Check event with correct tally
        System::assert_last_event(
            Event::FundingFinalized {
                funding_id,
                group_id,
                approved: true,
                total_ayes: 300 * 3 + 200 * 2,
                total_nays: 100 * 1,
            }
            .into(),
        );

        // Group balance should be increased
        assert_eq!(Template::spendable_balance(group_id), amount);
    });
}

#[test]
fn finalize_vote_not_approved_when_nays_majority() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;

        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));

        let funding_id = 0;

        assert_ok!(Template::vote(
            RuntimeOrigin::signed(2),
            funding_id,
            false,
            Conviction::Locked6x,
            100,
        ));

        assert_ok!(Template::vote(
            RuntimeOrigin::signed(3),
            funding_id,
            true,
            Conviction::Locked1x,
            100,
        ));

        let duration = Template::u64_to_block_saturated(30 * 24 * 60 * 60 / 6);
        System::set_block_number(System::block_number() + duration + 1);

        assert_ok!(Template::finalize_vote(
            RuntimeOrigin::signed(4),
            funding_id
        ));

        System::assert_last_event(
            Event::FundingFinalized {
                funding_id,
                group_id,
                approved: false,
                total_ayes: 100 * 1,
                total_nays: 100 * 6,
            }
            .into(),
        );

        // Funds not allocated
        assert_eq!(Template::spendable_balance(group_id), 0);
    });
}

#[test]
fn finalize_vote_with_no_votes_returns_correct_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;

        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));

        let funding_id = 0;

        let duration = Template::u64_to_block_saturated(30 * 24 * 60 * 60 / 6);
        System::set_block_number(System::block_number() + duration + 1);

        assert_ok!(Template::finalize_vote(
            RuntimeOrigin::signed(2),
            funding_id
        ));

        System::assert_last_event(
            Event::FundingFinalized {
                funding_id,
                group_id,
                approved: false,
                total_ayes: 0,
                total_nays: 0,
            }
            .into(),
        );

        // No funds added
        assert_eq!(Template::spendable_balance(group_id), 0);
    });
}

#[test]
fn finalize_vote_twice_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let proposer = 1;
        let group_id = 42;
        let amount = 10_000 * UNITS;
        let category = SpenderCategory::MediumSpender;

        let min_stake = Template::min_stake(&category);
        let _ = Balances::deposit_creating(&proposer, min_stake);

        assert_ok!(Template::propose_funding(
            RuntimeOrigin::signed(proposer),
            group_id,
            amount,
            Content::IPFS(b"QmHash".to_vec()),
            category,
        ));

        let funding_id = 0;

        let duration = Template::u64_to_block_saturated(30 * 24 * 60 * 60 / 6);
        System::set_block_number(System::block_number() + duration + 1);

        // First finalize
        assert_ok!(Template::finalize_vote(
            RuntimeOrigin::signed(2),
            funding_id
        ));

        // Second attempt
        assert_noop!(
            Template::finalize_vote(RuntimeOrigin::signed(3), funding_id),
            Error::<Test>::VoteStillLocked
        );
    });
}
