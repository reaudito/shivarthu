//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as DepartmentFunding;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use pallet_support::Content;
const SEED: u32 = 0;
use frame_support::{assert_noop, assert_ok};

#[benchmarks(
	     where T: pallet_departments::Config + frame_system::Config
)]
#[benchmarks]
mod benchmarks {
	use super::*;

	fn full_schelling_game<T: 'static + pallet::Config>(
		department_id: u64,
		start_block_number: u64,
		accounts: Vec<T::AccountId>,
	) {
		let start_block_number = DepartmentFunding::<T>::u64_to_block_saturated(start_block_number);
		let tipping_name = TippingName::SmallTipper;
		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::<T>::create_department_required_fund(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		));

		<frame_system::Pallet<T>>::set_block_number(start_block_number);
		assert_ok!(DepartmentFunding::<T>::apply_staking_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		let phase_data = DepartmentFunding::<T>::get_phase_data();

		for j in 4..30 {
			let stake = DepartmentFunding::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(DepartmentFunding::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				department_id,
				stake
			));
		}
		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));
		assert_ok!(DepartmentFunding::<T>::draw_jurors(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id,
			5
		));
		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[13].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			department_id,
			hash
		));

		<frame_system::Pallet<T>>::set_block_number(
			phase_data.evidence_length
				+ start_block_number
				+ phase_data.staking_length
				+ phase_data.commit_length,
		);

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		assert_ok!(DepartmentFunding::<T>::reveal_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			department_id,
			1,
			"salt2".as_bytes().to_vec()
		));

		assert_ok!(DepartmentFunding::<T>::reveal_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			department_id,
			1,
			"salt3".as_bytes().to_vec()
		));

		assert_ok!(DepartmentFunding::<T>::reveal_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			department_id,
			1,
			"salt5".as_bytes().to_vec()
		));

		assert_ok!(DepartmentFunding::<T>::reveal_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			department_id,
			0,
			"salt6".as_bytes().to_vec()
		));

		<frame_system::Pallet<T>>::set_block_number(
			phase_data.evidence_length
				+ start_block_number
				+ phase_data.staking_length
				+ phase_data.commit_length
				+ phase_data.vote_length,
		);

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		// assert_ok!(ProjectTips::<T>::add_incentive_count(
		// 	RawOrigin::Signed(accounts[14].clone()).into(),
		// 	project_id
		// ));
	}

	#[benchmark]
	fn create_department_required_fund() {
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;

		let account1 = account::<T::AccountId>("account1", 1, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(account1.clone()).into(),
			content_department
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		#[extrinsic_call]
		create_department_required_fund(
			RawOrigin::Signed(account1),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		);
	}

	#[benchmark]
	fn apply_staking_period() {
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;

		let account1 = account::<T::AccountId>("account1", 1, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(account1.clone()).into(),
			content_department
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::<T>::create_department_required_fund(
			RawOrigin::Signed(account1.clone()).into(),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		));

		let start_block_number = DepartmentFunding::<T>::u64_to_block_saturated(50);

		<frame_system::Pallet<T>>::set_block_number(start_block_number);

		#[extrinsic_call]
		apply_staking_period(RawOrigin::Signed(account1), department_id)
	}

	#[benchmark]
	fn apply_jurors() {
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;

		let account1 = account::<T::AccountId>("account1", 1, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(account1.clone()).into(),
			content_department
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::<T>::create_department_required_fund(
			RawOrigin::Signed(account1.clone()).into(),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		));

		let start_block_number = DepartmentFunding::<T>::u64_to_block_saturated(50);

		<frame_system::Pallet<T>>::set_block_number(start_block_number);
		assert_ok!(DepartmentFunding::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			department_id
		));

		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = DepartmentFunding::<T>::u64_to_balance_saturated(100);

		#[extrinsic_call]
		apply_jurors(RawOrigin::Signed(account2.clone()), department_id, stake)
	}

	#[benchmark]
	fn pass_period() {
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;

		let account1 = account::<T::AccountId>("account1", 1, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(account1.clone()).into(),
			content_department
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::<T>::create_department_required_fund(
			RawOrigin::Signed(account1.clone()).into(),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		));

		let start_block_number = DepartmentFunding::<T>::u64_to_block_saturated(50);

		<frame_system::Pallet<T>>::set_block_number(start_block_number);
		assert_ok!(DepartmentFunding::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			department_id
		));

		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = DepartmentFunding::<T>::u64_to_balance_saturated(100);

		let mut accounts = vec![];

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}

		for j in 4..30 {
			let stake = DepartmentFunding::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(DepartmentFunding::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				department_id.clone(),
				stake
			));
		}

		let phase_data = DepartmentFunding::<T>::get_phase_data();

		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		#[extrinsic_call]
		pass_period(RawOrigin::Signed(accounts[0].clone()), department_id)
	}

	#[benchmark]
	fn draw_jurors() {
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;

		let account1 = account::<T::AccountId>("account1", 1, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(account1.clone()).into(),
			content_department
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::<T>::create_department_required_fund(
			RawOrigin::Signed(account1.clone()).into(),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		));

		let start_block_number = DepartmentFunding::<T>::u64_to_block_saturated(50);

		<frame_system::Pallet<T>>::set_block_number(start_block_number);
		assert_ok!(DepartmentFunding::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			department_id
		));

		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = DepartmentFunding::<T>::u64_to_balance_saturated(100);

		let mut accounts = vec![];

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}

		for j in 4..30 {
			let stake = DepartmentFunding::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(DepartmentFunding::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				department_id.clone(),
				stake
			));
		}

		let phase_data = DepartmentFunding::<T>::get_phase_data();

		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		#[extrinsic_call]
		draw_jurors(RawOrigin::Signed(accounts[1].clone()), department_id, 5);
	}

	#[benchmark]
	fn commit_vote() {
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;

		let account1 = account::<T::AccountId>("account1", 1, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(account1.clone()).into(),
			content_department
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::<T>::create_department_required_fund(
			RawOrigin::Signed(account1.clone()).into(),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		));

		let start_block_number = DepartmentFunding::<T>::u64_to_block_saturated(50);

		<frame_system::Pallet<T>>::set_block_number(start_block_number);
		assert_ok!(DepartmentFunding::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			department_id
		));

		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = DepartmentFunding::<T>::u64_to_balance_saturated(100);

		let mut accounts = vec![];

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}

		for j in 4..30 {
			let stake = DepartmentFunding::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(DepartmentFunding::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				department_id.clone(),
				stake
			));
		}

		let phase_data = DepartmentFunding::<T>::get_phase_data();

		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		assert_ok!(DepartmentFunding::<T>::draw_jurors(
			RawOrigin::Signed(accounts[1].clone()).into(),
			department_id,
			5
		));

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());

		#[extrinsic_call]
		commit_vote(RawOrigin::Signed(accounts[4].clone()), department_id, hash);
	}

	#[benchmark]
	fn reveal_vote() {
		let department_id = 1;
		let tipping_name = TippingName::SmallTipper;

		let account1 = account::<T::AccountId>("account1", 1, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		let funding_needed = DepartmentFunding::<T>::u64_to_balance_saturated(10_000u64);
		// Dispatch a signed extrinsic.
		let department_account_id = 5;
		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(account1.clone()).into(),
			content_department
		));

		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(DepartmentFunding::<T>::create_department_required_fund(
			RawOrigin::Signed(account1.clone()).into(),
			department_id,
			content.clone(),
			tipping_name,
			funding_needed,
		));

		let start_block_number = DepartmentFunding::<T>::u64_to_block_saturated(50);

		<frame_system::Pallet<T>>::set_block_number(start_block_number);
		assert_ok!(DepartmentFunding::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			department_id
		));

		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = DepartmentFunding::<T>::u64_to_balance_saturated(100);

		let mut accounts = vec![];

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}

		for j in 4..30 {
			let stake = DepartmentFunding::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(DepartmentFunding::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				department_id.clone(),
				stake
			));
		}

		let phase_data = DepartmentFunding::<T>::get_phase_data();

		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		assert_ok!(DepartmentFunding::<T>::draw_jurors(
			RawOrigin::Signed(accounts[1].clone()).into(),
			department_id,
			5
		));

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());

		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[13].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			department_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
		assert_ok!(DepartmentFunding::<T>::commit_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			department_id,
			hash
		));

		<frame_system::Pallet<T>>::set_block_number(
			phase_data.evidence_length
				+ start_block_number
				+ phase_data.staking_length
				+ phase_data.commit_length,
		);

		assert_ok!(DepartmentFunding::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			department_id
		));

		#[extrinsic_call]
		reveal_vote(
			RawOrigin::Signed(accounts[4].clone()),
			department_id,
			1,
			"salt2".as_bytes().to_vec(),
		)
	}

	#[benchmark]
	fn add_incentive_count() {
		let mut accounts = vec![];
		let balance = DepartmentFunding::<T>::u64_to_balance_saturated(100000000000000);

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}
		let block_number = DepartmentFunding::<T>::u64_to_block_saturated(1000);
		<frame_system::Pallet<T>>::set_block_number(block_number);
		let department_id = 1;

		let content_department: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		assert_ok!(<pallet_departments::Pallet<T>>::create_department(
			RawOrigin::Signed(accounts[0].clone()).into(),
			content_department
		));
		full_schelling_game::<T>(department_id, 1000, accounts.clone());
		#[extrinsic_call]
		add_incentive_count(RawOrigin::Signed(accounts[14].clone()), department_id)
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
