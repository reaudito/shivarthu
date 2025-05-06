use super::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

/// Reputation scores that can be used for schelling game.

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ReputationScore {
    pub departments: BTreeMap<Department, i64>,
    pub total_score: i64,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, RuntimeDebug, TypeInfo)]
pub enum DepartmentType {
    Locality,
    Specialization,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, RuntimeDebug, TypeInfo)]
pub struct Department {
    pub name: Vec<u8>,
    pub department_type: DepartmentType,
    pub id: u64,
}

impl ReputationScore {
    pub fn new() -> ReputationScore {
        ReputationScore {
            departments: BTreeMap::new(),
            total_score: 0,
        }
    }

    pub fn add_department(&mut self, department: Department, score: i64) {
        self.departments.insert(department, score);
        self.total_score = self.total_score.checked_add(score).unwrap_or(i64::MAX);
    }

    pub fn update_department(&mut self, department: Department, score: i64) {
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

    pub fn get_department_score(&self, department: Department) -> Option<i64> {
        self.departments.get(&department).copied()
    }

    pub fn get_all_departments(&self) -> Vec<(Department, i64)> {
        self.departments
            .iter()
            .map(|(v, i)| (v.clone(), i.clone()))
            .collect()
    }

    pub fn add_score(&mut self, department: Department, amount: i64) {
        if let Some(score) = self.departments.get_mut(&department) {
            // Update department score with overflow check
            match score.checked_add(amount) {
                Some(new_score) => *score = new_score,
                None => {
                    if amount.is_negative() {
                        *score = i64::MIN;
                    } else {
                        *score = i64::MAX;
                    }
                }
            }

            // Update total score with overflow check
            match self.total_score.checked_add(amount) {
                Some(new_total) => self.total_score = new_total,
                None => {
                    if amount.is_negative() {
                        self.total_score = i64::MIN;
                    } else {
                        self.total_score = i64::MAX;
                    }
                }
            }
        }
    }

    pub fn get_total_score(&self) -> i64 {
        self.total_score
    }
}
