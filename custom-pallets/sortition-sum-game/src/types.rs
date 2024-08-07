use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

type CitizenId = u64;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SumTreeName<AccountId, BlockNumber> {
	ProfileValidation { citizen_address: AccountId, block_number: BlockNumber },
	PositiveExternality { user_address: AccountId, block_number: BlockNumber },
	DepartmentRequiredFund { department_required_fund_id: u64, block_number: BlockNumber },
	ProjectTips { project_id: u64, block_number: BlockNumber },
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SortitionSumTree<AccountId> {
	pub k: u64,
	pub stack: Vec<u64>,
	pub nodes: Vec<u64>,
	pub ids_to_node_indexes: BTreeMap<AccountId, u64>, // citizen id, node index
	pub node_indexes_to_ids: BTreeMap<u64, AccountId>, // node index, citizen id
}
