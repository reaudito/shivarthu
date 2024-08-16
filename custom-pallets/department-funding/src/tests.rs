use crate::types::{Incentives, TippingName};
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_schelling_game_shared::types::Period;
use pallet_sortition_sum_game::types::SumTreeName;
use pallet_support::Content;
use pallet_support::WhenDetails;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let account_id = 1;
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;
		let funding_needed = 10_000u64.into();
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(department_account_id),
			content_department.clone()
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::create_department_required_fund(
			RuntimeOrigin::signed(account_id),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed
		));

		// Check that the department fund was stored correctly
		let department_fund_id = DepartmentFunding::next_department_required_fund_id() - 1;
		let stored_fund = DepartmentFunding::department_required_funds(department_fund_id).unwrap();

		assert_eq!(stored_fund.department_id, department_id);
		assert_eq!(stored_fund.content, content);
		assert_eq!(stored_fund.tipping_name, tipping_name);
		assert_eq!(stored_fund.funding_needed, funding_needed);
		assert_eq!(stored_fund.creator, account_id);

		// Verify that the correct event was emitted
		System::assert_last_event(
			Event::DepartmentFundCreated {
				account: account_id,
				department_required_fund_id: department_fund_id,
			}
			.into(),
		);
	});
}

#[test]
fn create_department_required_fund_fails_if_funding_more_than_tipping_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(department_account_id),
			content_department.clone()
		));
		let account_id = 1;
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;
		let funding_needed = 20_000u64.into();

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		// Dispatch the extrinsic, should fail due to funding_needed exceeding max tipping value
		assert_noop!(
			DepartmentFunding::create_department_required_fund(
				RuntimeOrigin::signed(account_id),
				department_id,
				content,
				tipping_name,
				funding_needed
			),
			Error::<Test>::FundingMoreThanTippingValue
		);
	});
}

#[test]
fn create_department_required_fund_fails_if_department_does_not_exist() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let account_id = 1;
		let department_id = 999; // Assuming this department ID does not exist
		let tipping_name = TippingName::SmallTipper;
		let funding_needed = 50u64.into();

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		// Dispatch the extrinsic, should fail because the department does not exist
		assert_noop!(
			DepartmentFunding::create_department_required_fund(
				RuntimeOrigin::signed(account_id),
				department_id,
				content,
				tipping_name,
				funding_needed
			),
			<pallet_departments::Error<Test>>::DepartmentDontExists
		);
	});
}

#[test]
fn apply_staking_period_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let account_id = 1;
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;
		let funding_needed = 10_000u64.into();
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(department_account_id),
			content_department.clone()
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::create_department_required_fund(
			RuntimeOrigin::signed(account_id),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed
		));

		System::set_block_number(50);

		// Dispatch the extrinsic
		assert_ok!(DepartmentFunding::apply_staking_period(
			RuntimeOrigin::signed(account_id),
			department_id
		));

		// Verify that the correct event was emitted
		let block_number = System::block_number();

		System::assert_last_event(
			Event::StakingPeriodStarted {
				department_required_fund_id: department_id,
				block_number,
			}
			.into(),
		);
	});
}

#[test]
fn apply_staking_period_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let account_id = 1;
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;
		let funding_needed = 10_000u64.into();
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(department_account_id),
			content_department.clone()
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::create_department_required_fund(
			RuntimeOrigin::signed(account_id),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed
		));

		System::set_block_number(50);

		// Dispatch the extrinsic
		assert_ok!(DepartmentFunding::apply_staking_period(
			RuntimeOrigin::signed(account_id),
			department_id
		));

		// Verify that the correct event was emitted
		let block_number = System::block_number();

		System::assert_last_event(
			Event::StakingPeriodStarted {
				department_required_fund_id: department_id,
				block_number,
			}
			.into(),
		);

		System::set_block_number(80);
		assert_noop!(
			DepartmentFunding::apply_staking_period(
				RuntimeOrigin::signed(account_id),
				department_id,
			),
			Error::<Test>::FundingStatusProcessing
		);
	});
}

fn full_schelling_game_func(department_required_fund_id: u64, start_block_number: u64) {
	System::set_block_number(start_block_number);
	assert_ok!(DepartmentFunding::apply_staking_period(
		RuntimeOrigin::signed(2),
		department_required_fund_id
	));

	let phase_data = DepartmentFunding::get_phase_data();

	for j in 4..30 {
		assert_ok!(DepartmentFunding::apply_jurors(
			RuntimeOrigin::signed(j),
			department_required_fund_id,
			j * 100
		));
	}

	assert_noop!(
		DepartmentFunding::draw_jurors(RuntimeOrigin::signed(5), department_required_fund_id, 5),
		<pallet_schelling_game_shared::Error<Test>>::PeriodDontMatch
	);
	assert_noop!(
		DepartmentFunding::pass_period(RuntimeOrigin::signed(5), department_required_fund_id),
		<pallet_schelling_game_shared::Error<Test>>::StakingPeriodNotOver
	);

	System::set_block_number(start_block_number + phase_data.staking_length);

	assert_ok!(DepartmentFunding::pass_period(
		RuntimeOrigin::signed(5),
		department_required_fund_id
	));

	assert_ok!(DepartmentFunding::draw_jurors(
		RuntimeOrigin::signed(5),
		department_required_fund_id,
		5
	));

	let block_number = DepartmentFunding::validation_block(department_required_fund_id).unwrap();

	let key = SumTreeName::DepartmentRequiredFund { department_required_fund_id, block_number };

	let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
	assert_eq!(5, draws_in_round);

	let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
	assert_eq!(vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)], drawn_jurors);

	assert_ok!(DepartmentFunding::pass_period(
		RuntimeOrigin::signed(5),
		department_required_fund_id
	));

	let period = SchellingGameShared::get_period(key.clone());

	assert_eq!(Some(Period::Commit), period);

	assert_ok!(DepartmentFunding::unstaking(RuntimeOrigin::signed(5), department_required_fund_id));

	let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
	assert_noop!(
		DepartmentFunding::commit_vote(RuntimeOrigin::signed(6), department_required_fund_id, hash),
		<pallet_schelling_game_shared::Error<Test>>::JurorDoesNotExists
	);
	let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
	assert_ok!(DepartmentFunding::commit_vote(
		RuntimeOrigin::signed(4),
		department_required_fund_id,
		hash
	));

	// You can replace vote within the commit period.
	let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
	assert_ok!(DepartmentFunding::commit_vote(
		RuntimeOrigin::signed(4),
		department_required_fund_id,
		hash
	));

	let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
	assert_ok!(DepartmentFunding::commit_vote(
		RuntimeOrigin::signed(7),
		department_required_fund_id,
		hash
	));

	let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
	assert_ok!(DepartmentFunding::commit_vote(
		RuntimeOrigin::signed(13),
		department_required_fund_id,
		hash
	));

	let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
	assert_ok!(DepartmentFunding::commit_vote(
		RuntimeOrigin::signed(14),
		department_required_fund_id,
		hash
	));

	let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
	assert_ok!(DepartmentFunding::commit_vote(
		RuntimeOrigin::signed(15),
		department_required_fund_id,
		hash
	));

	assert_noop!(
		DepartmentFunding::pass_period(RuntimeOrigin::signed(5), department_required_fund_id),
		<pallet_schelling_game_shared::Error<Test>>::CommitPeriodNotOver
	);
	System::set_block_number(
		phase_data.evidence_length
			+ start_block_number
			+ phase_data.staking_length
			+ phase_data.commit_length,
	);
	assert_ok!(DepartmentFunding::pass_period(
		RuntimeOrigin::signed(5),
		department_required_fund_id
	));

	assert_noop!(
		DepartmentFunding::reveal_vote(
			RuntimeOrigin::signed(4),
			department_required_fund_id,
			2,
			"salt2".as_bytes().to_vec()
		),
		<pallet_schelling_game_shared::Error<Test>>::CommitDoesNotMatch
	);

	assert_ok!(DepartmentFunding::reveal_vote(
		RuntimeOrigin::signed(4),
		department_required_fund_id,
		1,
		"salt2".as_bytes().to_vec()
	));

	assert_ok!(DepartmentFunding::reveal_vote(
		RuntimeOrigin::signed(7),
		department_required_fund_id,
		1,
		"salt3".as_bytes().to_vec()
	));

	assert_ok!(DepartmentFunding::reveal_vote(
		RuntimeOrigin::signed(14),
		department_required_fund_id,
		1,
		"salt5".as_bytes().to_vec()
	));

	assert_ok!(DepartmentFunding::reveal_vote(
		RuntimeOrigin::signed(15),
		department_required_fund_id,
		0,
		"salt6".as_bytes().to_vec()
	));

	assert_noop!(
		DepartmentFunding::pass_period(RuntimeOrigin::signed(5), department_required_fund_id),
		<pallet_schelling_game_shared::Error<Test>>::VotePeriodNotOver
	);
	System::set_block_number(
		phase_data.evidence_length
			+ start_block_number
			+ phase_data.staking_length
			+ phase_data.commit_length
			+ phase_data.vote_length,
	);
	assert_ok!(DepartmentFunding::pass_period(
		RuntimeOrigin::signed(5),
		department_required_fund_id
	));

	assert_noop!(
		DepartmentFunding::add_incentive_count(
			RuntimeOrigin::signed(13),
			department_required_fund_id
		),
		<pallet_schelling_game_shared::Error<Test>>::VoteNotRevealed
	);
	assert_ok!(DepartmentFunding::add_incentive_count(
		RuntimeOrigin::signed(14),
		department_required_fund_id
	));
	assert_ok!(DepartmentFunding::add_incentive_count(
		RuntimeOrigin::signed(15),
		department_required_fund_id
	));
}

#[test]
fn schelling_game_incentives_get_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let startblock1 = 1 * (6 * 30 * 24 * 60 * 60) / 6;

		let startblock2 = 2 * (6 * 30 * 24 * 60 * 60) / 6;

		let startblock3 = 3 * (6 * 30 * 24 * 60 * 60) / 6;

		let startblock4 = 4 * (6 * 30 * 24 * 60 * 60) / 6;

		let account_id = 1;
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;
		let funding_needed = 10_000u64.into();
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(department_account_id),
			content_department.clone()
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::create_department_required_fund(
			RuntimeOrigin::signed(account_id),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed
		));

		full_schelling_game_func(department_id, startblock1);

		let incentive_count = DepartmentFunding::incentives_count(14).unwrap();

		let incentive_count_eq: Incentives<Test> = Incentives {
			number_of_games: 1,
			winner: 1,
			loser: 0,
			total_stake: 14 * 100,
			start: WhenDetails { block: 2592200, time: 0 },
		};

		assert_eq!(incentive_count, incentive_count_eq);
	})
}
