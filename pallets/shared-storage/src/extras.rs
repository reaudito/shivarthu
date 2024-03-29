use crate::*;

use shared_storage_link::SharedStorageLink;

impl<T: Config> SharedStorageLink for Pallet<T> {
	type AccountId = AccountIdOf<T>;

	fn check_citizen_is_approved_link(address: Self::AccountId) -> DispatchResult {
		Self::check_citizen_is_approved(address)
	}
	fn get_approved_citizen_count_link() -> u64 {
		Self::get_approved_citizen_count()
	}

	fn set_positive_externality_link(address: Self::AccountId, score: i64) -> DispatchResult {
		Self::set_positive_externality(address, score)
	}
}

impl<T: Config> Pallet<T> {
	pub(super) fn check_citizen_is_approved(address: T::AccountId) -> DispatchResult {
		let members = ApprovedCitizenAddress::<T>::get();

		match members.binary_search(&address) {
			Ok(_index) => Ok(()),
			Err(_) => Err(Error::<T>::CitizenNotApproved.into()),
		}
	}

	pub(super) fn get_approved_citizen_count() -> u64 {
		let members = ApprovedCitizenAddress::<T>::get();
		members.len() as u64
	}

	pub(super) fn set_positive_externality(address: T::AccountId, score: Score) -> DispatchResult {
		PositiveExternalityScore::<T>::insert(address, score);
		Ok(())
	}
}
