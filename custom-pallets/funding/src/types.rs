use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_arithmetic::Perbill;

use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum MajorityType {
    Simple,
    Super,
}

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
            self.approvals += 1;
        } else {
            self.rejections += 1;
        }
    }

    pub fn is_passed(&self, total_voters: usize, majority_type: MajorityType) -> bool {
        match majority_type {
            MajorityType::Simple => self.approvals > self.rejections,
            MajorityType::Super => {
                let required = Perbill::from_percent(66) * (total_voters as u32);
                self.approvals >= required
            }
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub struct FundingProposal<Balance> {
    pub amount: Balance,
    pub group_id: u64,
    pub created_at: u64,
    pub proposal_id: u64,
}
