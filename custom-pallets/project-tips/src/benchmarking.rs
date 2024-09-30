//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::types::{Incentives, TippingName};
#[allow(unused)]
use crate::Pallet as ProjectTips;
use frame_benchmarking::v2::*;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use log::log;
use log::{info, warn};
use pallet_schelling_game_shared::types::Period;
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
	fn full_schelling_game<T: 'static + pallet::Config>(
		who_ask_tipper: T::AccountId,
		start_block_number: u64,
		accounts: Vec<T::AccountId>,
	) {
		let start_block_number = ProjectTips::<T>::u64_to_block_saturated(start_block_number);
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

		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(who_ask_tipper.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let project_ids = ProjectTips::<T>::get_projects_from_accounts(who_ask_tipper.clone());

		let project_id = project_ids.last().unwrap();

		let project_id = *project_id;

		// info!("project_id {:?}", project_id);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&who_ask_tipper, balance);

		assert_ok!(ProjectTips::<T>::apply_staking_period(
			RawOrigin::Signed(who_ask_tipper.clone()).into(),
			project_id
		));

		let stake = ProjectTips::<T>::u64_to_balance_saturated(100);
		let phase_data = ProjectTips::<T>::get_phase_data();

		for j in 4..30 {
			let stake = ProjectTips::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(ProjectTips::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				project_id,
				stake
			));
		}
		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));
		assert_ok!(ProjectTips::<T>::draw_jurors(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id,
			5
		));

		let key = SumTreeName::ProjectTips { project_id, block_number: start_block_number };

		let drawn_jurors = <pallet_schelling_game_shared::Pallet<T>>::drawn_jurors(key.clone());
		// info!("Draw Jurors {:?}", drawn_jurors);
		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));

		let period = <pallet_schelling_game_shared::Pallet<T>>::get_period(key.clone());

		// info!("period {:?}", period);

		// assert_eq!(Some(Period::Commit), period);

		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[13].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			project_id,
			hash
		));

		<frame_system::Pallet<T>>::set_block_number(
			phase_data.evidence_length
				+ start_block_number
				+ phase_data.staking_length
				+ phase_data.commit_length,
		);

		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			project_id,
			1,
			"salt2".as_bytes().to_vec()
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			project_id,
			1,
			"salt3".as_bytes().to_vec()
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			project_id,
			1,
			"salt5".as_bytes().to_vec()
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			project_id,
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

		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));

		// assert_ok!(ProjectTips::<T>::add_incentive_count(
		// 	RawOrigin::Signed(accounts[14].clone()).into(),
		// 	project_id
		// ));
	}

	fn full_schelling_game2<T: 'static + pallet::Config>(
		who_ask_tipper: T::AccountId,
		start_block_number: u64,
		accounts: Vec<T::AccountId>,
	) {
		let start_block_number = ProjectTips::<T>::u64_to_block_saturated(start_block_number);
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

		assert_ok!(ProjectTips::<T>::create_project(
			RawOrigin::Signed(who_ask_tipper.clone()).into(),
			2,
			content,
			tipping_name,
			funding_needed
		));
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		let project_ids = ProjectTips::<T>::get_projects_from_accounts(who_ask_tipper.clone());

		let project_id = project_ids.last().unwrap();

		let project_id = *project_id;

		// info!("project_id {:?}", project_id);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&who_ask_tipper, balance);

		assert_ok!(ProjectTips::<T>::apply_staking_period(
			RawOrigin::Signed(who_ask_tipper.clone()).into(),
			project_id
		));

		let stake = ProjectTips::<T>::u64_to_balance_saturated(100);
		let phase_data = ProjectTips::<T>::get_phase_data();

		for j in 4..30 {
			let stake = ProjectTips::<T>::u64_to_balance_saturated(j * 100);
			assert_ok!(ProjectTips::<T>::apply_jurors(
				RawOrigin::Signed(accounts[(j) as usize].clone()).into(),
				project_id,
				stake
			));
		}
		<frame_system::Pallet<T>>::set_block_number(start_block_number + phase_data.staking_length);

		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));
		assert_ok!(ProjectTips::<T>::draw_jurors(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id,
			5
		));

		let key = SumTreeName::ProjectTips { project_id, block_number: start_block_number };

		let drawn_jurors = <pallet_schelling_game_shared::Pallet<T>>::drawn_jurors(key.clone());
		// info!("Draw Jurors {:?}", drawn_jurors);
		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));

		let period = <pallet_schelling_game_shared::Pallet<T>>::get_period(key.clone());

		// info!("period {:?}", period);

		// assert_eq!(Some(Period::Commit), period);

		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[13].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			project_id,
			hash
		));

		let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
		assert_ok!(ProjectTips::<T>::commit_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			project_id,
			hash
		));

		<frame_system::Pallet<T>>::set_block_number(
			phase_data.evidence_length
				+ start_block_number
				+ phase_data.staking_length
				+ phase_data.commit_length,
		);

		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[4].clone()).into(),
			project_id,
			1,
			"salt2".as_bytes().to_vec()
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[7].clone()).into(),
			project_id,
			1,
			"salt3".as_bytes().to_vec()
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[14].clone()).into(),
			project_id,
			1,
			"salt5".as_bytes().to_vec()
		));

		assert_ok!(ProjectTips::<T>::reveal_vote(
			RawOrigin::Signed(accounts[15].clone()).into(),
			project_id,
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

		assert_ok!(ProjectTips::<T>::pass_period(
			RawOrigin::Signed(accounts[0].clone()).into(),
			project_id
		));

		assert_ok!(ProjectTips::<T>::add_incentive_count(
			RawOrigin::Signed(accounts[14].clone()).into(),
			project_id
		));
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

	#[benchmark]
	fn add_incentive_count() {
		let mut accounts = vec![];
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}
		let block_number = ProjectTips::<T>::u64_to_block_saturated(1000);
		<frame_system::Pallet<T>>::set_block_number(block_number);
		full_schelling_game::<T>(accounts[1].clone(), 1000, accounts.clone());
		#[extrinsic_call]
		add_incentive_count(RawOrigin::Signed(accounts[14].clone()), 1)
	}

	#[benchmark]
	fn get_incentives() {
		let mut accounts = vec![];
		let balance = ProjectTips::<T>::u64_to_balance_saturated(100000000000000);

		for j in 4..50 {
			let account_number = account::<T::AccountId>("apply-juror-account", j, SEED);
			let _ = <T as pallet::Config>::Currency::deposit_creating(&account_number, balance);
			accounts.push(account_number);
		}

		for x in 1..21 {
			let block_number = ProjectTips::<T>::u64_to_block_saturated(x * 1000);
			<frame_system::Pallet<T>>::set_block_number(block_number);
			full_schelling_game2::<T>(accounts[x as usize].clone(), x * 1000, accounts.clone());
		}
		#[extrinsic_call]
		get_incentives(RawOrigin::Signed(accounts[14].clone()))
	}

	impl_benchmark_test_suite!(ProjectTips, crate::mock::new_test_ext(), crate::mock::Test);
}
