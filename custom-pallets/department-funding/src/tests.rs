use crate::types::TippingName;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_support::Content;

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
