use crate::conviction::Conviction;
use codec::Decode;
use codec::Encode;
use codec::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Amount of votes and capital placed in delegation for an account.
#[derive(
    Encode, Decode, Default, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen,
)]
pub struct Delegations<Balance> {
    /// The number of votes (this is post-conviction).
    pub votes: Balance,
    /// The amount of raw capital, used for the support.
    pub capital: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct Vote<Balance> {
    pub aye: bool,
    pub conviction: Conviction,
    pub balance: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct VoteRecord<Balance, BlockNumber> {
    pub vote: Vote<Balance>,
    pub expiry: BlockNumber, // expiry block for lock
}
