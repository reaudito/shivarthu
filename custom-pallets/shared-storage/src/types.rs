use super::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

/// Reputation scores that can be used for schelling game.

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ReputationScore {
    pub departments: BTreeMap<u64, i64>, // Department Id, Score
    pub total_score: i64,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, RuntimeDebug, TypeInfo)]
pub enum DepartmentType {
    District,
    Specialization,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, RuntimeDebug, TypeInfo)]
pub struct Department {
    pub name: BoundedVec<u8, MaxNameLength>,
    pub department_type: DepartmentType,
    pub id: u64,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Group {
    pub id: u64,
    pub name: BoundedVec<u8, MaxNameLength>,
    pub specialization_departments: BoundedVec<u64, MaxDepartmentsPerGroup>,
    pub district_departments: BoundedVec<u64, MaxDepartmentsPerGroup>,
}

impl ReputationScore {
    pub fn new() -> ReputationScore {
        ReputationScore {
            departments: BTreeMap::new(),
            total_score: 0,
        }
    }

    pub fn add_department(&mut self, department_id: u64, score: i64) {
        self.departments.insert(department_id, score);
        self.total_score = self.total_score.checked_add(score).unwrap_or(i64::MAX);
    }

    pub fn update_department(&mut self, department_id: u64, score: i64) {
        if let Some(existing_score) = self.departments.get_mut(&department_id) {
            self.total_score = self
                .total_score
                .checked_sub(*existing_score)
                .unwrap_or(i64::MIN);
            *existing_score = score;
            self.total_score = self.total_score.checked_add(score).unwrap_or(i64::MAX);
        } else {
            self.add_department(department_id, score);
        }
    }

    pub fn get_department_score(&self, department_id: u64) -> Option<i64> {
        self.departments.get(&department_id).copied()
    }

    pub fn get_all_departments(&self) -> Vec<(u64, i64)> {
        self.departments
            .iter()
            .map(|(v, i)| (v.clone(), i.clone()))
            .collect()
    }

    pub fn add_score(&mut self, department_id: u64, amount: i64) {
        if let Some(score) = self.departments.get_mut(&department_id) {
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
