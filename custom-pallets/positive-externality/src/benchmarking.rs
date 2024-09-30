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

	impl_benchmark_test_suite!(PositiveExternality, crate::mock::new_test_ext(), crate::mock::Test);
}
