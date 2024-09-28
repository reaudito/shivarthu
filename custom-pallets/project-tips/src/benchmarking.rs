//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::types::{Incentives, TippingName};
#[allow(unused)]
use crate::Pallet as ProjectTips;
use frame_benchmarking::v2::*;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_schelling_game_shared::PeriodName;
use pallet_schelling_game_shared::StakingStartTime;
use pallet_sortition_sum_game::SortitionSumTrees;
use pallet_support::Content;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks {
	use super::*;

	fn assert_last_event<T: 'static + pallet::Config>(
		generic_event: <T as pallet::Config>::RuntimeEvent,
	) {
		frame_system::Pallet::<T>::assert_last_event(generic_event.into());
	}

	#[benchmark]
	fn create_project() {
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::<T>::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let stake_required = tipping_value.stake_required;
		let funding_needed = max_tipping_value;

		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		create_project(RawOrigin::Signed(caller), 5, content.clone(), tipping_name, funding_needed);
	}

	#[benchmark]
	fn apply_staking_period() {
		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::<T>::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let sub_value = ProjectTips::<T>::u64_to_balance_saturated(100);
		let funding_needed = max_tipping_value - sub_value;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		let account1 = account::<T::AccountId>("account1", 1, SEED);
		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(account1.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		#[extrinsic_call]
		apply_staking_period(RawOrigin::Signed(account1), 1);
		let now = <frame_system::Pallet<T>>::block_number();
		assert_last_event::<T>(
			Event::StakinPeriodStarted { project_id: 1, block_number: now }.into(),
		);
	}

	#[benchmark]
	fn apply_jurors() {
		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::<T>::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let sub_value = ProjectTips::<T>::u64_to_balance_saturated(100);
		let funding_needed = max_tipping_value - sub_value;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		let account1 = account::<T::AccountId>("account1", 1, SEED);
		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(account1.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		assert_ok!(ProjectTips::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			1
		));

		let account2 = account::<T::AccountId>("apply-juror-account", 2, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = ProjectTips::<T>::u64_to_balance_saturated(100);

		#[extrinsic_call]
		apply_jurors(RawOrigin::Signed(account2.clone()), 1, stake);
		let now = <frame_system::Pallet<T>>::block_number();
		let block_number = ProjectTips::<T>::get_block_number_of_schelling_game(1).unwrap();

		assert_last_event::<T>(
			Event::ApplyJurors { project_id: 1, block_number, account: account2 }.into(),
		);
	}

	#[benchmark]
	fn pass_period() {
		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::<T>::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let sub_value = ProjectTips::<T>::u64_to_balance_saturated(100);
		let funding_needed = max_tipping_value - sub_value;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		let account1 = account::<T::AccountId>("account1", 1, SEED);
		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(account1.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		assert_ok!(ProjectTips::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			1
		));

		let account2 = account::<T::AccountId>("apply-juror-account", 2, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = ProjectTips::<T>::u64_to_balance_saturated(100);
		let phase_data = ProjectTips::<T>::get_phase_data();

		let mut accounts = vec![];

		for j in 4..30 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			accounts.push(account_number.clone());
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);

			assert_ok!(ProjectTips::<T>::apply_jurors(
				RawOrigin::Signed(account_number).into(),
				1,
				(j * 100).into()
			));
		}

		let now = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(now + phase_data.staking_length);

		#[extrinsic_call]
		pass_period(RawOrigin::Signed(accounts[0].clone()), 1);
	}

	#[benchmark]
	fn draw_jurors() {
		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::<T>::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let sub_value = ProjectTips::<T>::u64_to_balance_saturated(100);
		let funding_needed = max_tipping_value - sub_value;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		let account1 = account::<T::AccountId>("account1", 1, SEED);
		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(account1.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		assert_ok!(ProjectTips::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			1
		));

		let account2 = account::<T::AccountId>("apply-juror-account", 2, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = ProjectTips::<T>::u64_to_balance_saturated(100);
		let phase_data = ProjectTips::<T>::get_phase_data();

		let mut accounts = vec![];

		for j in 4..30 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			accounts.push(account_number.clone());
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);

			assert_ok!(ProjectTips::<T>::apply_jurors(
				RawOrigin::Signed(account_number).into(),
				1,
				(j * 100).into()
			));
		}

		let now = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(now + phase_data.staking_length);

		assert_ok!(ProjectTips::<T>::pass_period(RawOrigin::Signed(accounts[0].clone()).into(), 1));

		#[extrinsic_call]
		draw_jurors(RawOrigin::Signed(accounts[1].clone()), 1, 5);
	}

	#[benchmark]
	fn commit_vote() {
		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::<T>::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let sub_value = ProjectTips::<T>::u64_to_balance_saturated(100);
		let funding_needed = max_tipping_value - sub_value;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		let account1 = account::<T::AccountId>("account1", 1, SEED);
		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(account1.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		assert_ok!(ProjectTips::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			1
		));

		let account2 = account::<T::AccountId>("apply-juror-account", 2, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = ProjectTips::<T>::u64_to_balance_saturated(100);
		let phase_data = ProjectTips::<T>::get_phase_data();

		let mut accounts = vec![];

		for j in 4..30 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			accounts.push(account_number.clone());
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);

			assert_ok!(ProjectTips::<T>::apply_jurors(
				RawOrigin::Signed(account_number).into(),
				1,
				(j * 100).into()
			));
		}

		let now = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(now + phase_data.staking_length);

		assert_ok!(ProjectTips::<T>::pass_period(RawOrigin::Signed(accounts[0].clone()).into(), 1));
		assert_ok!(ProjectTips::<T>::draw_jurors(
			RawOrigin::Signed(accounts[1].clone()).into(),
			1,
			5
		));
		assert_ok!(ProjectTips::<T>::pass_period(RawOrigin::Signed(accounts[0].clone()).into(), 1));
		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
		#[extrinsic_call]
		commit_vote(RawOrigin::Signed(accounts[0].clone()), 1, hash);
	}

	#[benchmark]
	fn reveal_vote() {
		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::<T>::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let sub_value = ProjectTips::<T>::u64_to_balance_saturated(100);
		let funding_needed = max_tipping_value - sub_value;
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		let account1 = account::<T::AccountId>("account1", 1, SEED);
		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(account1.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account1, balance);

		assert_ok!(ProjectTips::<T>::apply_staking_period(
			RawOrigin::Signed(account1.clone()).into(),
			1
		));

		let account2 = account::<T::AccountId>("apply-juror-account", 2, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);

		let stake = ProjectTips::<T>::u64_to_balance_saturated(100);
		let phase_data = ProjectTips::<T>::get_phase_data();

		let mut accounts = vec![];

		for j in 4..30 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			accounts.push(account_number.clone());
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);

			assert_ok!(ProjectTips::<T>::apply_jurors(
				RawOrigin::Signed(account_number).into(),
				1,
				(j * 100).into()
			));
		}

		let now = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(now + phase_data.staking_length);

		assert_ok!(ProjectTips::<T>::pass_period(RawOrigin::Signed(accounts[0].clone()).into(), 1));
		assert_ok!(ProjectTips::<T>::draw_jurors(
			RawOrigin::Signed(accounts[1].clone()).into(),
			1,
			5
		));
		assert_ok!(ProjectTips::<T>::pass_period(RawOrigin::Signed(accounts[0].clone()).into(), 1));
		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[0].clone()).into(),
			1,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[7 - 4].clone()).into(),
			1,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[13 - 4].clone()).into(),
			1,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[14 - 4].clone()).into(),
			1,
			hash
		));

		let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[15 - 4].clone()).into(),
			1,
			hash
		));

		<frame_system::Pallet<T>>::set_block_number(
			phase_data.evidence_length + now + phase_data.staking_length + phase_data.commit_length,
		);

		assert_ok!(ProjectTips::<T>::pass_period(RawOrigin::Signed(accounts[0].clone()).into(), 1));

		#[extrinsic_call]
		reveal_vote(RawOrigin::Signed(accounts[0].clone()), 1, 1, "salt2".as_bytes().to_vec())
	}

	impl_benchmark_test_suite!(ProjectTips, crate::mock::new_test_ext(), crate::mock::Test);
}
