use super::*;
use frame_support::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

pub const FIRST_DEPARTMENT_ID: DepartmentId = 1;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct DepartmentDetails<T: Config> {
	pub created: WhoAndWhenOf<T>,
	pub department_id: DepartmentId,
	pub content: Content,
	pub department_admin: T::AccountId,
}

impl<T: Config> DepartmentDetails<T> {
	pub fn new(
		department_id: DepartmentId,
		content: Content,
		department_admin: T::AccountId,
	) -> Self {
		DepartmentDetails {
			created: new_who_and_when::<T>(department_admin.clone()),
			department_id,
			content,
			department_admin,
		}
	}
}
