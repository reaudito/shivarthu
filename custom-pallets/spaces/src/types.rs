use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

use super::*;

use pallet_support::{new_who_and_when, WhoAndWhenOf};

pub const FIRST_SPACE_ID: u64 = 1;
pub const RESERVED_SPACE_COUNT: u64 = 1000;

/// Information about a space's owner, its' content, visibility and custom permissions.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Space<T: Config> {
	/// Unique sequential identifier of a space. Examples of space ids: `1`, `2`, `3`, and so on.
	pub id: SpaceId,

	pub created: WhoAndWhenOf<T>,
	/// True, if the content of this space was edited.
	pub edited: bool,

	/// The current owner of a given space.
	pub owner: T::AccountId,

	// The next fields can be updated by the owner:
	pub content: Content,

	/// Hidden field is used to recommend to end clients (web and mobile apps) that a particular
	/// space and its' posts should not be shown.
	pub hidden: bool,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct SpaceUpdate {
	pub content: Option<Content>,
	pub hidden: Option<bool>,
}

impl<T: Config> Space<T> {
	pub fn new(id: SpaceId, created_by: T::AccountId, content: Content) -> Self {
		Space {
			id,
			created: new_who_and_when::<T>(created_by.clone()),
			edited: false,
			owner: created_by,
			content,
			hidden: false,
		}
	}

	pub fn is_owner(&self, account: &T::AccountId) -> bool {
		self.owner == *account
	}
}
