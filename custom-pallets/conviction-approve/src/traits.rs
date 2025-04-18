use codec::Decode;
use codec::Encode;
use codec::HasCompact;
use codec::MaxEncodedLen;
use frame_support::pallet_prelude::DispatchError;
use frame_support::traits::{ReservableCurrency, VoteTally};
use frame_support::Parameter;
use pallet_referenda::{Deposit, TallyOf, TracksInfo};
use scale_info::prelude::fmt::Debug;
use scale_info::TypeInfo;
use sp_runtime::traits::Member;

use crate::types::ReferendumStates;

pub trait ReferendumTrait<AccountId> {
    type Index: From<u32>
        + Parameter
        + Member
        + Ord
        + PartialOrd
        + Copy
        + HasCompact
        + MaxEncodedLen;
    type Proposal: Parameter + Member + MaxEncodedLen;
    type ReferendumInfo: Eq + PartialEq + Debug + Encode + Decode + TypeInfo + Clone;
    type Preimages;
    type Call;
    type Moment;

    fn create_proposal(proposal_call: Self::Call) -> Self::Proposal;
    fn submit_proposal(caller: AccountId, proposal: Self::Proposal) -> Result<u32, DispatchError>;
    fn get_referendum_info(index: Self::Index) -> Option<Self::ReferendumInfo>;
    fn handle_referendum_info(infos: Self::ReferendumInfo) -> Option<ReferendumStates>;
    fn referendum_count() -> Self::Index;
    fn get_decision_period(index: Self::Index) -> Result<u128, DispatchError>;
}
