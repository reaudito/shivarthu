use super::*;
use frame_support::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

/// Reputation scores that can be used for schelling game.

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ReputationScore {
    pub departments: BTreeMap<Vec<u8>, i64>,
    pub total_score: i64,
}

impl ReputationScore {
    pub fn new() -> ReputationScore {
        ReputationScore {
            departments: BTreeMap::new(),
            total_score: 0,
        }
    }

    pub fn add_department(&mut self, department: Vec<u8>, score: i64) {
        self.departments.insert(department, score);
        self.total_score = self.total_score.checked_add(score).unwrap_or(i64::MAX);
    }

    pub fn update_department(&mut self, department: Vec<u8>, score: i64) {
        if let Some(existing_score) = self.departments.get_mut(&department) {
            self.total_score = self
                .total_score
                .checked_sub(*existing_score)
                .unwrap_or(i64::MIN);
            *existing_score = score;
            self.total_score = self.total_score.checked_add(score).unwrap_or(i64::MAX);
        } else {
            self.add_department(department, score);
        }
    }

    pub fn get_department_score(&self, department: Vec<u8>) -> Option<i64> {
        self.departments.get(&department).copied()
    }

    pub fn get_all_departments(&self) -> Vec<(Vec<u8>, i64)> {
        self.departments
            .iter()
            .map(|(v, i)| (v.clone(), i.clone()))
            .collect()
    }

    pub fn add_score(&mut self, department: Vec<u8>, amount: i64) {
        if let Some(score) = self.departments.get_mut(&department) {
            *score = score.checked_add(amount).unwrap_or(i64::MAX);
            self.total_score = self.total_score.checked_add(amount).unwrap_or(i64::MAX);
        }
    }

    pub fn subtract_score(&mut self, department: Vec<u8>, amount: i64) -> bool {
        if let Some(score) = self.departments.get_mut(&department) {
            if *score >= amount {
                *score = score.checked_sub(amount).unwrap_or(i64::MIN);
                self.total_score = self.total_score.checked_sub(amount).unwrap_or(i64::MIN);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_total_score(&self) -> i64 {
        self.total_score
    }
}
