#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::dispatch::DispatchResult;
use frame_support::pallet_prelude::DispatchError;
use sp_std::prelude::*;

pub trait DepartmentsLink {
	type DepartmentId;
	fn check_department_exists(department_id: Self::DepartmentId) -> DispatchResult;
}
