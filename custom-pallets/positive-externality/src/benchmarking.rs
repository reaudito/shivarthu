//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PositiveExternality;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use pallet_support::Content;
const SEED: u32 = 0;
use frame_support::{assert_noop, assert_ok};

#[benchmarks]
mod benchmarks {
	use super::*;

	fn assert_last_event<T: 'static + pallet::Config>(
		generic_event: <T as pallet::Config>::RuntimeEvent,
	) {
		frame_system::Pallet::<T>::assert_last_event(generic_event.into());
	}

	#[benchmark]
	fn create_positive_externality_post() {
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);

		let account1 = account::<T::AccountId>("create-positive-externality", 1, SEED);

		#[extrinsic_call]
		create_positive_externality_post(RawOrigin::Signed(account1.clone()), content);
	}

	#[benchmark]
	fn set_validate_positive_externality() {
		let account1 = account::<T::AccountId>("set-validate", 1, SEED);

		#[extrinsic_call]
		set_validate_positive_externality(RawOrigin::Signed(account1.clone()), true);
	}

	#[benchmark]
	fn apply_staking_period() {
		let account1 = account::<T::AccountId>("set-validate", 1, SEED);
		assert_ok!(PositiveExternality::<T>::set_validate_positive_externality(
			RawOrigin::Signed(account1.clone()).into(),
			true
		));
		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = PositiveExternality::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);
		#[extrinsic_call]
		apply_staking_period(RawOrigin::Signed(account2.clone()), account1.clone())
	}

	#[benchmark]
	fn apply_jurors() {
		let account1 = account::<T::AccountId>("set-validate", 1, SEED);
		assert_ok!(PositiveExternality::<T>::set_validate_positive_externality(
			RawOrigin::Signed(account1.clone()).into(),
			true
		));
		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = PositiveExternality::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);
		assert_ok!(PositiveExternality::<T>::apply_staking_period(
			RawOrigin::Signed(account2.clone()).into(),
			account1.clone()
		));

		let account3 = account::<T::AccountId>("apply-juror-account", 3, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account3, balance);

		let stake = PositiveExternality::<T>::u64_to_balance_saturated(100);

		#[extrinsic_call]
		apply_jurors(RawOrigin::Signed(account2.clone()), account1.clone(), stake);
	}

	#[benchmark]
	fn pass_period() {
		let account1 = account::<T::AccountId>("set-validate", 1, SEED);
		assert_ok!(PositiveExternality::<T>::set_validate_positive_externality(
			RawOrigin::Signed(account1.clone()).into(),
			true
		));
		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = PositiveExternality::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);
		assert_ok!(PositiveExternality::<T>::apply_staking_period(
			RawOrigin::Signed(account2.clone()).into(),
			account1.clone()
		));

		let account3 = account::<T::AccountId>("apply-juror-account", 3, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account3, balance);

		let stake = PositiveExternality::<T>::u64_to_balance_saturated(100);

		let mut accounts = vec![];

		for j in 4..30 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			accounts.push(account_number.clone());
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);

			assert_ok!(PositiveExternality::<T>::apply_jurors(
				RawOrigin::Signed(account_number).into(),
				account1.clone(),
				(j * 100).into()
			));
		}

		let phase_data = PositiveExternality::<T>::get_phase_data();

		let now = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(now + phase_data.staking_length);

		#[extrinsic_call]
		pass_period(RawOrigin::Signed(accounts[0].clone()), account1.clone());
	}

	#[benchmark]
	fn draw_jurors() {
		let account1 = account::<T::AccountId>("set-validate", 1, SEED);
		assert_ok!(PositiveExternality::<T>::set_validate_positive_externality(
			RawOrigin::Signed(account1.clone()).into(),
			true
		));
		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = PositiveExternality::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);
		assert_ok!(PositiveExternality::<T>::apply_staking_period(
			RawOrigin::Signed(account2.clone()).into(),
			account1.clone()
		));

		let account3 = account::<T::AccountId>("apply-juror-account", 3, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account3, balance);

		let stake = PositiveExternality::<T>::u64_to_balance_saturated(100);

		let mut accounts = vec![];

		for j in 4..2000 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			accounts.push(account_number.clone());
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);

			assert_ok!(PositiveExternality::<T>::apply_jurors(
				RawOrigin::Signed(account_number).into(),
				account1.clone(),
				(j * 100).into()
			));
		}

		let phase_data = PositiveExternality::<T>::get_phase_data();

		let now = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(now + phase_data.staking_length);

		assert_ok!(PositiveExternality::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			account1.clone()
		));

		#[extrinsic_call]
		draw_jurors(RawOrigin::Signed(accounts[1].clone()), account1.clone(), 5);
	}

	#[benchmark]
	fn commit_vote() {
		let account1 = account::<T::AccountId>("set-validate", 1, SEED);
		assert_ok!(PositiveExternality::<T>::set_validate_positive_externality(
			RawOrigin::Signed(account1.clone()).into(),
			true
		));
		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = PositiveExternality::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);
		assert_ok!(PositiveExternality::<T>::apply_staking_period(
			RawOrigin::Signed(account2.clone()).into(),
			account1.clone()
		));

		let account3 = account::<T::AccountId>("apply-juror-account", 3, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account3, balance);

		let stake = PositiveExternality::<T>::u64_to_balance_saturated(100);

		let mut accounts = vec![];

		for j in 4..30 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			accounts.push(account_number.clone());
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);

			assert_ok!(PositiveExternality::<T>::apply_jurors(
				RawOrigin::Signed(account_number).into(),
				account1.clone(),
				(j * 100).into()
			));
		}

		let phase_data = PositiveExternality::<T>::get_phase_data();

		let now = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(now + phase_data.staking_length);

		assert_ok!(PositiveExternality::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			account1.clone()
		));

		assert_ok!(PositiveExternality::<T>::draw_jurors(
			RawOrigin::Signed(accounts[1].clone()).into(),
			account1.clone(),
			5
		));

		assert_ok!(PositiveExternality::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			account1.clone()
		));
		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());

		#[extrinsic_call]
		commit_vote(RawOrigin::Signed(accounts[0].clone()), account1.clone(), hash);
	}

	#[benchmark]
	fn reveal_vote() {
		let mut accounts: Vec<T::AccountId> = vec![];
		let balance = PositiveExternality::<T>::u64_to_balance_saturated(100000000000000);

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}

		let account1 = account::<T::AccountId>("set-validate", 1, SEED);

		let user_to_calculate = account1.clone();
		assert_ok!(PositiveExternality::<T>::set_validate_positive_externality(
			RawOrigin::Signed(account1.clone()).into(),
			true
		));
		let account2 = account::<T::AccountId>("stake-account", 2, SEED);

		let balance = PositiveExternality::<T>::u64_to_balance_saturated(100000000000000);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account2, balance);
		assert_ok!(PositiveExternality::<T>::apply_staking_period(
			RawOrigin::Signed(account2.clone()).into(),
			user_to_calculate.clone()
		));

		let account3 = account::<T::AccountId>("apply-juror-account", 3, SEED);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&account3, balance);

		let stake = PositiveExternality::<T>::u64_to_balance_saturated(100);

		for j in 4..30 {
			let stake = PositiveExternality::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(PositiveExternality::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				user_to_calculate.clone(),
				stake
			));
		}

		let phase_data = PositiveExternality::<T>::get_phase_data();

		let start_block_number = <frame_system::Pallet<T>>::block_number();

		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		assert_ok!(PositiveExternality::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			user_to_calculate.clone()
		));

		assert_ok!(PositiveExternality::<T>::draw_jurors(
			RawOrigin::Signed(accounts[1].clone()).into(),
			user_to_calculate.clone(),
			5
		));

		assert_ok!(PositiveExternality::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			user_to_calculate.clone()
		));

		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(PositiveExternality::<T>::commit_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			user_to_calculate.clone(),
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(PositiveExternality::<T>::commit_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			user_to_calculate.clone(),
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(PositiveExternality::<T>::commit_vote(
			RawOrigin::Signed(accounts[13].clone()).into(),
			user_to_calculate.clone(),
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(PositiveExternality::<T>::commit_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			user_to_calculate.clone(),
			hash
		));

		let hash = sp_io::hashing::keccak_256("3salt6".as_bytes());
		assert_ok!(PositiveExternality::<T>::commit_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			user_to_calculate.clone(),
			hash
		));

		<frame_system::Pallet<T>>::set_block_number(
			phase_data.evidence_length
				+ start_block_number
				+ phase_data.staking_length
				+ phase_data.commit_length,
		);

		assert_ok!(PositiveExternality::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			user_to_calculate.clone()
		));

		// assert_ok!(PositiveExternality::<T>::reveal_vote(
		// 	RawOrigin::Signed(accounts[4].clone()).into(),
		// 	user_to_calculate.clone(),
		// 	1,
		// 	"salt2".as_bytes().to_vec()
		// ));

		#[extrinsic_call]
		reveal_vote(
			RawOrigin::Signed(accounts[4].clone()),
			user_to_calculate.clone(),
			1,
			"salt2".as_bytes().to_vec(),
		)
	}

	impl_benchmark_test_suite!(PositiveExternality, crate::mock::new_test_ext(), crate::mock::Test);
}
