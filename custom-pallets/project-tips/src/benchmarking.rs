//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::types::{Incentives, TippingName};
#[allow(unused)]
use crate::Pallet as ProjectTips;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use pallet_support::Content;

#[benchmarks]
mod benchmarks {
	use super::*;

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

	impl_benchmark_test_suite!(ProjectTips, crate::mock::new_test_ext(), crate::mock::Test);
}
