use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_support::Content;

#[test]
fn create_department_works() {
	new_test_ext().execute_with(|| {
		// // Go past genesis block so events get deposited
		System::set_block_number(1);
		let account_id = 1;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(account_id),
			content.clone()
		));

		// Check that the department was stored correctly
		let department_id = Departments::next_department_id() - 1;
		let stored_department = Departments::departments(department_id).unwrap();
		assert_eq!(stored_department.department_id, department_id);
		assert_eq!(stored_department.content, content);
		assert_eq!(stored_department.department_admin, account_id);

		// Verify that the correct event was emitted
		System::assert_last_event(
			Event::DepartmentCreated { account: account_id, department_id }.into(),
		);
	});
}

#[test]
fn add_member_to_department_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let admin_account_id = 1;
		let new_member_account_id = 2;
		let new_member_2_account_id = 3;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		// Create a department first
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(admin_account_id),
			content.clone()
		));

		// Get the department ID
		let department_id = Departments::next_department_id() - 1;

		// Verify that the correct event was emitted for department creation
		System::assert_last_event(
			Event::DepartmentCreated { account: admin_account_id, department_id }.into(),
		);

		// Add a new member to the department
		assert_ok!(Departments::add_member_to_department(
			RuntimeOrigin::signed(admin_account_id),
			department_id,
			new_member_account_id
		));

		// Check that the member was added correctly
		let stored_accounts = Departments::department_accounts(department_id).unwrap();
		assert!(stored_accounts.contains(&new_member_account_id));

		// Verify that the correct event was emitted for adding a member
		System::assert_last_event(
			Event::MemberAdded { new_member: new_member_account_id, department_id }.into(),
		);

		// Add a new member to the department
		assert_ok!(Departments::add_member_to_department(
			RuntimeOrigin::signed(admin_account_id),
			department_id,
			new_member_2_account_id
		));

		// Check that the member was added correctly
		let stored_accounts = Departments::department_accounts(department_id).unwrap();
		assert!(stored_accounts.contains(&new_member_2_account_id));

		// Verify that the correct event was emitted for adding a member
		System::assert_last_event(
			Event::MemberAdded { new_member: new_member_2_account_id, department_id }.into(),
		);
	});
}

#[test]
fn add_member_to_department_fails_if_not_admin() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let non_admin_account_id = 2;
		let new_member_account_id = 3;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		// Create a department with account_id 1 as the admin
		assert_ok!(Departments::create_department(RuntimeOrigin::signed(1), content.clone()));

		// Get the department ID
		let department_id = Departments::next_department_id() - 1;

		// Try to add a member with a non-admin account, should fail
		assert_noop!(
			Departments::add_member_to_department(
				RuntimeOrigin::signed(non_admin_account_id),
				department_id,
				new_member_account_id
			),
			Error::<Test>::NotAdmin
		);
	});
}

#[test]
fn add_member_to_department_fails_if_already_member() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let admin_account_id = 1;
		let new_member_account_id = 2;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		// Create a department first
		assert_ok!(Departments::create_department(
			RuntimeOrigin::signed(admin_account_id),
			content.clone()
		));

		// Get the department ID
		let department_id = Departments::next_department_id() - 1;

		// Add the member for the first time
		assert_ok!(Departments::add_member_to_department(
			RuntimeOrigin::signed(admin_account_id),
			department_id,
			new_member_account_id
		));

		// Try to add the same member again, should fail
		assert_noop!(
			Departments::add_member_to_department(
				RuntimeOrigin::signed(admin_account_id),
				department_id,
				new_member_account_id
			),
			Error::<Test>::AlreadyMember
		);
	});
}
