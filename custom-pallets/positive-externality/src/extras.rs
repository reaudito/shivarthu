use frame_support::dispatch::DispatchResult;

use super::*;

impl<T: Config> Post<T> {
	pub fn new(id: PostId, created_by: T::AccountId, content: Content) -> Self {
		Post {
			id,
			created: new_who_and_when::<T>(created_by.clone()),
			edited: false,
			owner: created_by,
			content,
			hidden: false,
			upvotes_count: 0,
			downvotes_count: 0,
		}
	}

	pub fn ensure_owner(&self, account: &T::AccountId) -> DispatchResult {
		ensure!(self.is_owner(account), Error::<T>::NotAPostOwner);
		Ok(())
	}

	pub fn is_owner(&self, account: &T::AccountId) -> bool {
		self.owner == *account
	}
}

impl<T: Config> Pallet<T> {
	pub(super) fn get_phase_data() -> PhaseData<T> {
		T::SchellingGameSharedSource::create_phase_data(50, 5, 3, 100, (100, 100))
	}

	pub fn ensure_validation_on_positive_externality(account: T::AccountId) -> DispatchResult {
		let bool_data = Validate::<T>::get(account);
		ensure!(bool_data == true, Error::<T>::ValidationPositiveExternalityIsOff);

		Ok(())
	}

	pub fn ensure_min_stake_positive_externality(account: T::AccountId) -> DispatchResult {
		let stake = StakeBalance::<T>::get(account);
		let min_stake = MinimumStake::<T>::get();
		// println!("stake {:?}", stake);
		// println!("min stake {:?}", min_stake);
		ensure!(stake >= min_stake, Error::<T>::LessThanMinStake);

		Ok(())
	}

	pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}

	pub(super) fn u64_to_block_saturated(input: u64) -> BlockNumberOf<T> {
		input.saturated_into::<BlockNumberOf<T>>()
	}

	pub(super) fn get_drawn_jurors(user_to_calculate: T::AccountId) -> Vec<(T::AccountId, u64)> {
		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		T::SchellingGameSharedSource::get_drawn_jurors(key)
	}

	// Block code start

	pub fn get_evidence_period_end_block(user_to_calculate: T::AccountId) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_evidence_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn get_staking_period_end_block(user_to_calculate: T::AccountId) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_staking_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn get_drawing_period_end(user_to_calculate: T::AccountId) -> (u64, u64, bool) {
		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};
		let phase_data = Self::get_phase_data();

		let result =
			T::SchellingGameSharedSource::get_drawing_period_end_helper_link(key, phase_data);
		result
	}

	pub fn get_commit_period_end_block(user_to_calculate: T::AccountId) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_commit_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn get_vote_period_end_block(user_to_calculate: T::AccountId) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_vote_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn selected_as_juror(user_to_calculate: T::AccountId, who: T::AccountId) -> bool {
		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		let result = T::SchellingGameSharedSource::selected_as_juror_helper_link(key, who);
		result
	}

	pub fn has_user_staked(user_to_calculate: T::AccountId, who: T::AccountId) -> bool {
		let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		let result = T::SchellingGameSharedSource::has_user_staked(key, who);
		result
	}

pub fn user_staked_value(user_to_calculate: T::AccountId, who: T::AccountId) -> u64 {
	let pe_block_number = <ValidationBlock<T>>::get(user_to_calculate.clone());

		let key = SumTreeName::PositiveExternality {
			user_address: user_to_calculate,
			block_number: pe_block_number.clone(),
		};

		let result = T::SchellingGameSharedSource::user_staked_value(key, who);
		result
}

	// Block code end

	pub fn post_by_address_length(user: T::AccountId) -> u64 {
		PostByAddresss::<T>::get(user).len().try_into().unwrap()
	}

	pub fn paginate_posts_by_address(
		user: T::AccountId,
		page: u64,
		page_size: u64,
	) -> Option<Vec<u64>> {
		let all_posts = PostByAddresss::<T>::get(user);
		let start = (page - 1) * page_size;

		if start >= all_posts.len() as u64 {
			// If start exceeds available posts, return None (no more pages).
			return None;
		}

		let end = (start + page_size).min(all_posts.len() as u64);
		Some(all_posts[start as usize..end as usize].to_vec())
	}

	pub fn paginate_posts_by_address_latest(
		user: T::AccountId,
		page: u64,
		page_size: u64,
	) -> Option<Vec<u64>> {
		let mut all_posts = PostByAddresss::<T>::get(user);
		all_posts.reverse();

		let start = (page - 1) * page_size;

		if start >= all_posts.len() as u64 {
			// If start exceeds available posts, return None (no more pages).
			return None;
		}

		let end = (start + page_size).min(all_posts.len() as u64);
		Some(all_posts[start as usize..end as usize].to_vec())
	}

	pub fn validation_list_length() -> u64 {
		match <ValidationList<T>>::get() {
			Some(value) => value.len().try_into().unwrap(),
			None => 0,
		}
	}

	pub fn validation_list_latest(page: u64, page_size: u64) -> Option<Vec<T::AccountId>> {
		let mut all_accounts = match <ValidationList<T>>::get() {
			Some(value) => value,
			None => vec![],
		};
		all_accounts.reverse();

		let start = (page - 1) * page_size;

		if start >= all_accounts.len() as u64 {
			// If start exceeds available posts, return None (no more pages).
			return None;
		}

		let end = (start + page_size).min(all_accounts.len() as u64);
		Some(all_accounts[start as usize..end as usize].to_vec())
	}
}
