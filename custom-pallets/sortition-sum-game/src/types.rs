use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

type _CitizenId = u64;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SumTreeName<AccountId, BlockNumber> {
	ProfileValidation { citizen_address: AccountId, block_number: BlockNumber },
	PositiveExternality { user_address: AccountId, block_number: BlockNumber },
	DepartmentRequiredFund { department_required_fund_id: u64, block_number: BlockNumber },
	ProjectTips { project_id: u64, block_number: BlockNumber },
}

/// SortitionSumTree Struct:
/// `k`: Represents the number of children each non-leaf node has in the tree. For example, in a binary tree, k=2.
/// `stack`: A stack used to store vacant nodes for efficient reuse when values are set to 0.
/// `nodes`: The list of nodes in the tree. The root node holds the sum of all values (i.e., all the tokens in this case), and the leaf nodes store the actual stake values.
/// `ids_to_node_indexes`: A mapping from account IDs (participants in the game) to their corresponding node indexes in the tree.
/// `node_indexes_to_ids`: A reverse mapping from node indexes to account IDs.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SortitionSumTree<AccountId> {
	pub k: u64,
	pub stack: Vec<u64>,
	pub nodes: Vec<u64>,
	pub ids_to_node_indexes: BTreeMap<AccountId, u64>, // citizen id, node index
	pub node_indexes_to_ids: BTreeMap<u64, AccountId>, // node index, citizen id
}
