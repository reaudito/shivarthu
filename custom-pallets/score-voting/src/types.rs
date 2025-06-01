use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use pallet_support::Content;
use scale_info::TypeInfo;
use sp_arithmetic::{traits::CheckedDiv, FixedU128};

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum BountyStatus {
    Active,
    Finalized,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct BountyInfo<AccountId, Balance, Moment> {
    /// Recipient of the bounty
    pub recipient: AccountId,

    /// Optional amount to be released if approved
    pub amount: Option<Balance>,

    /// The group this bounty belongs to
    pub group_id: u64,

    /// Start time of the vote
    pub vote_start: Moment,

    /// Approval statistics
    pub approval: MajorityApproval,

    /// Status of the bounty
    pub status: BountyStatus,

    /// Content associated with the bounty proposal
    pub content: Content,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum MajorityType {
    Simple,
    Super,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, Default)]
pub struct MajorityApproval {
    pub approvals: u32,
    pub rejections: u32,
}

impl MajorityApproval {
    pub fn new() -> Self {
        Self {
            approvals: 0,
            rejections: 0,
        }
    }

    pub fn vote(&mut self, approve: bool) {
        if approve {
            self.approvals = self.approvals.saturating_add(1);
        } else {
            self.rejections = self.rejections.saturating_add(1);
        }
    }

    /// Check if bounty can be released, given electorate and turnout
    pub fn can_release(&self, electorate: u32, turnout: u32, majority_type: MajorityType) -> bool {
        match majority_type {
            MajorityType::Simple => self.approvals > self.rejections,
            MajorityType::Super => {
                // Convert to FixedU128
                let ayes = FixedU128::from_u32(self.approvals);
                let nays = FixedU128::from_u32(self.rejections);
                let electorate = FixedU128::from_u32(electorate);
                let turnout = FixedU128::from_u32(turnout);

                // Calculate sqrt safely (use zero or one if None to avoid panic/div by zero)
                let sqrt_electorate = electorate.try_sqrt().unwrap_or_else(FixedU128::zero);
                let sqrt_turnout = turnout.try_sqrt().unwrap_or_else(FixedU128::one);

                // left = nays / sqrt(turnout)
                let left = nays
                    .checked_div(&sqrt_turnout)
                    .unwrap_or_else(FixedU128::zero);
                // right = ayes / sqrt(electorate)
                let right = ayes
                    .checked_div(&sqrt_electorate)
                    .unwrap_or_else(FixedU128::zero);

                left < right
            }
        }
    }
}
