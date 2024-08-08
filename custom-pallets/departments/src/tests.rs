use crate::{mock::*, Error, Event, Something};
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
