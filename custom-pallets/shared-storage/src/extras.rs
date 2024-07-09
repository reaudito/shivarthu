use crate::*;

use frame_support::pallet_prelude::DispatchResult;
use trait_shared_storage::SharedStorageLink;

impl<T: Config> SharedStorageLink for Pallet<T> {
    type AccountId = AccountIdOf<T>;

    fn check_citizen_is_approved_link(address: Self::AccountId) -> DispatchResult {
        Self::check_citizen_is_approved(address)
    }
    fn get_approved_citizen_count_link() -> u64 {
        Self::get_approved_citizen_count()
    }

    fn set_positive_externality_link(address: Self::AccountId, score: i64) -> DispatchResult {
        Self::set_positive_externality(address, score)
    }

    fn add_reputation_score_to_department(
        address: T::AccountId,
        department: Vec<u8>,
        amount: i64,
    ) -> DispatchResult {
        Self::add_reputation_score_to_department(address, department, amount)
    }
    fn subtract_reputation_score_from_department(
        address: T::AccountId,
        department: Vec<u8>,
        amount: i64,
    ) -> DispatchResult {
        Self::subtract_reputation_score_from_department(address, department, amount)
    }

    fn get_department_reputation_score(address: T::AccountId, department: Vec<u8>) -> Option<i64> {
        Self::get_department_reputation_score(address, department)
    }
    fn get_total_reputation_score(address: T::AccountId) -> i64 {
        Self::get_total_reputation_score(address)
    }
}

impl<T: Config> Pallet<T> {
    pub(super) fn check_citizen_is_approved(address: T::AccountId) -> DispatchResult {
        let members = ApprovedCitizenAddress::<T>::get();

        match members.binary_search(&address) {
            Ok(_index) => Ok(()),
            Err(_) => Err(Error::<T>::CitizenNotApproved.into()),
        }
    }

    pub(super) fn get_approved_citizen_count() -> u64 {
        let members = ApprovedCitizenAddress::<T>::get();
        members.len() as u64
    }

    pub(super) fn set_positive_externality(address: T::AccountId, score: Score) -> DispatchResult {
        PositiveExternalityScore::<T>::insert(address, score);
        Ok(())
    }

    pub fn set_department_reputation_score(
        address: T::AccountId,
        department: Vec<u8>,
        score: i64,
    ) -> DispatchResult {
        ReputationScoreOfAccount::<T>::mutate(address, |reputation_score| {
            if let Some(reputation_score) = reputation_score.as_mut() {
                reputation_score.add_department(department.clone(), score);
            } else {
                *reputation_score = Some(ReputationScore::new());
                reputation_score
                    .as_mut()
                    .unwrap()
                    .add_department(department.clone(), score);
            }
        });
        Ok(())
    }

    pub fn update_department_reputation_score(
        address: T::AccountId,
        department: Vec<u8>,
        score: i64,
    ) -> DispatchResult {
        ReputationScoreOfAccount::<T>::mutate(address, |reputation_score| {
            if let Some(reputation_score) = reputation_score.as_mut() {
                reputation_score.update_department(department.clone(), score);
            } else {
                *reputation_score = Some(ReputationScore::new());
                reputation_score
                    .as_mut()
                    .unwrap()
                    .update_department(department.clone(), score);
            }
        });
        Ok(())
    }

    pub fn add_reputation_score_to_department(
        address: T::AccountId,
        department: Vec<u8>,
        amount: i64,
    ) -> DispatchResult {
        ReputationScoreOfAccount::<T>::mutate(address, |reputation_score| {
            if let Some(reputation_score) = reputation_score.as_mut() {
                reputation_score.add_score(department.clone(), amount);
            } else {
                *reputation_score = Some(ReputationScore::new());
                reputation_score
                    .as_mut()
                    .unwrap()
                    .add_score(department.clone(), amount);
            }
        });
        Ok(())
    }

    pub fn subtract_reputation_score_from_department(
        address: T::AccountId,
        department: Vec<u8>,
        amount: i64,
    ) -> DispatchResult {
        ReputationScoreOfAccount::<T>::mutate(address, |reputation_score| {
            if let Some(reputation_score) = reputation_score.as_mut() {
                reputation_score.subtract_score(department.clone(), amount);
            } else {
                *reputation_score = Some(ReputationScore::new());
                reputation_score
                    .as_mut()
                    .unwrap()
                    .subtract_score(department.clone(), amount);
            }
        });
        Ok(())
    }

    pub fn get_department_reputation_score(
        address: T::AccountId,
        department: Vec<u8>,
    ) -> Option<i64> {
        ReputationScoreOfAccount::<T>::get(address)
            .and_then(|reputation_score| reputation_score.get_department_score(department.clone()))
    }

    pub fn get_all_department_reputation_scores(address: T::AccountId) -> Vec<(Vec<u8>, i64)> {
        ReputationScoreOfAccount::<T>::get(address)
            .map(|reputation_score| {
                reputation_score
                    .get_all_departments()
                    .iter()
                    .map(|(v, i)| (v.clone(), i.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }
    pub fn get_total_reputation_score(address: T::AccountId) -> i64 {
        ReputationScoreOfAccount::<T>::get(address)
            .map_or(0, |reputation_score| reputation_score.get_total_score())
    }
}
