
//! Autogenerated weights for `pallet_project_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `amiya`, CPU: `12th Gen Intel(R) Core(TM) i7-12650H`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --wasm-execution
// compiled
// --pallet
// pallet-project-tips
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// custom-pallets/project-tips/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_project_tips`.
pub trait WeightInfo {
	fn create_project() -> Weight;
	fn apply_staking_period() -> Weight;
	fn apply_jurors() -> Weight;
	fn pass_period() -> Weight;
}

/// Weights for `pallet_project_tips` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ProjectTips::NextProjectId` (r:1 w:1)
	/// Proof: `ProjectTips::NextProjectId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ProjectTips::AccountProjects` (r:1 w:1)
	/// Proof: `ProjectTips::AccountProjects` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ProjectTips::Projects` (r:0 w:1)
	/// Proof: `ProjectTips::Projects` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_project() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10`
		//  Estimated: `3475`
		// Minimum execution time: 11_481_000 picoseconds.
		Weight::from_parts(12_353_000, 3475)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `ProjectTips::Projects` (r:1 w:0)
	/// Proof: `ProjectTips::Projects` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ProjectTips::ValidationBlock` (r:1 w:1)
	/// Proof: `ProjectTips::ValidationBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `SchellingGameShared::PeriodName` (r:1 w:1)
	/// Proof: `SchellingGameShared::PeriodName` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SortitionSumGame::SortitionSumTrees` (r:1 w:1)
	/// Proof: `SortitionSumGame::SortitionSumTrees` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::StakingStartTime` (r:0 w:1)
	/// Proof: `SchellingGameShared::StakingStartTime` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn apply_staking_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `439`
		//  Estimated: `3904`
		// Minimum execution time: 40_028_000 picoseconds.
		Weight::from_parts(41_183_000, 3904)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `ProjectTips::ValidationBlock` (r:1 w:0)
	/// Proof: `ProjectTips::ValidationBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::PeriodName` (r:1 w:0)
	/// Proof: `SchellingGameShared::PeriodName` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `SortitionSumGame::SortitionSumTrees` (r:1 w:1)
	/// Proof: `SortitionSumGame::SortitionSumTrees` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn apply_jurors() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465`
		//  Estimated: `3930`
		// Minimum execution time: 38_091_000 picoseconds.
		Weight::from_parts(39_011_000, 3930)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ProjectTips::ValidationBlock` (r:1 w:0)
	/// Proof: `ProjectTips::ValidationBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::PeriodName` (r:1 w:1)
	/// Proof: `SchellingGameShared::PeriodName` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::StakingStartTime` (r:1 w:0)
	/// Proof: `SchellingGameShared::StakingStartTime` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn pass_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3767`
		// Minimum execution time: 19_778_000 picoseconds.
		Weight::from_parts(20_996_000, 3767)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `ProjectTips::NextProjectId` (r:1 w:1)
	/// Proof: `ProjectTips::NextProjectId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ProjectTips::AccountProjects` (r:1 w:1)
	/// Proof: `ProjectTips::AccountProjects` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ProjectTips::Projects` (r:0 w:1)
	/// Proof: `ProjectTips::Projects` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_project() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10`
		//  Estimated: `3475`
		// Minimum execution time: 11_481_000 picoseconds.
		Weight::from_parts(12_353_000, 3475)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `ProjectTips::Projects` (r:1 w:0)
	/// Proof: `ProjectTips::Projects` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ProjectTips::ValidationBlock` (r:1 w:1)
	/// Proof: `ProjectTips::ValidationBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `SchellingGameShared::PeriodName` (r:1 w:1)
	/// Proof: `SchellingGameShared::PeriodName` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SortitionSumGame::SortitionSumTrees` (r:1 w:1)
	/// Proof: `SortitionSumGame::SortitionSumTrees` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::StakingStartTime` (r:0 w:1)
	/// Proof: `SchellingGameShared::StakingStartTime` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn apply_staking_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `439`
		//  Estimated: `3904`
		// Minimum execution time: 40_028_000 picoseconds.
		Weight::from_parts(41_183_000, 3904)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `ProjectTips::ValidationBlock` (r:1 w:0)
	/// Proof: `ProjectTips::ValidationBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::PeriodName` (r:1 w:0)
	/// Proof: `SchellingGameShared::PeriodName` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `SortitionSumGame::SortitionSumTrees` (r:1 w:1)
	/// Proof: `SortitionSumGame::SortitionSumTrees` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn apply_jurors() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465`
		//  Estimated: `3930`
		// Minimum execution time: 38_091_000 picoseconds.
		Weight::from_parts(39_011_000, 3930)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ProjectTips::ValidationBlock` (r:1 w:0)
	/// Proof: `ProjectTips::ValidationBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::PeriodName` (r:1 w:1)
	/// Proof: `SchellingGameShared::PeriodName` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SchellingGameShared::StakingStartTime` (r:1 w:0)
	/// Proof: `SchellingGameShared::StakingStartTime` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn pass_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3767`
		// Minimum execution time: 19_778_000 picoseconds.
		Weight::from_parts(20_996_000, 3767)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
