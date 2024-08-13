#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::dispatch::DispatchResult;
use frame_support::pallet_prelude::DispatchError;
use sp_std::prelude::*;

pub trait DepartmentsLink {
	type DepartmentId;
	type AccountId;
	fn check_department_exists(department_id: Self::DepartmentId) -> DispatchResult;
	fn check_member_is_admin(
		who: Self::AccountId,
		department_id: Self::DepartmentId,
	) -> DispatchResult;
}
