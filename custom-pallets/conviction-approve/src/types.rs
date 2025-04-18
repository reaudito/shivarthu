use codec::Decode;
use codec::Encode;
use codec::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub enum ReferendumStates {
    Ongoing,
    Approved,
    Rejected,
    //Cancelled,
    //Timeout,
    //Killed,
}
