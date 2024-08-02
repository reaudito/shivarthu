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
}