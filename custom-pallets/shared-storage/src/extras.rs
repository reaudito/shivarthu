use crate::*;

use frame_support::pallet_prelude::DispatchResult;
use trait_shared_storage::SharedStorageLink;

impl<T: Config> SharedStorageLink for Pallet<T> {
    type AccountId = AccountIdOf<T>;

    fn add_approved_citizen_address(new_member: Self::AccountId) -> DispatchResult {
        Self::add_approved_citizen_address(new_member)
    }
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
        department_id: u64,
        amount: i64,
    ) -> DispatchResult {
        Self::add_reputation_score_to_department(address, department_id, amount)
    }

    fn get_department_reputation_score(address: T::AccountId, department_id: u64) -> Option<i64> {
        Self::get_department_reputation_score(address, department_id)
    }
    fn get_total_reputation_score(address: T::AccountId) -> i64 {
        Self::get_total_reputation_score(address)
    }
    fn is_member_in_group_district(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<bool, DispatchError> {
        Self::is_member_in_group_district(group_id, member)
    }

    fn is_member_in_group_specialization(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<bool, DispatchError> {
        Self::is_member_in_group_specialization(group_id, member)
    }

    fn is_member_and_score_in_group_specialization(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<(bool, i64), DispatchError> {
        Self::is_member_and_score_in_group_specialization(group_id, member)
    }

    fn are_district_departments_empty(group_id: u64) -> Result<bool, DispatchError> {
        Self::are_district_departments_empty(group_id)
    }
    fn are_specialization_departments_empty(group_id: u64) -> Result<bool, DispatchError> {
        Self::are_specialization_departments_empty(group_id)
    }

    fn is_member_in_group_district_and_specialization(
        group_id: u64,
        member: Self::AccountId,
    ) -> Result<bool, DispatchError> {
        Self::is_member_in_group_district_and_specialization(group_id, member)
    }
}

impl<T: Config> Pallet<T> {
    pub(super) fn add_approved_citizen_address(new_member: T::AccountId) -> DispatchResult {
        let mut members = ApprovedCitizenAddress::<T>::get();

        match members.binary_search(&new_member) {
            Ok(_) => Err(Error::<T>::AlreadyMember.into()),
            Err(index) => {
                members.insert(index, new_member.clone());
                ApprovedCitizenAddress::<T>::put(members);
                Ok(())
            }
        }
    }
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
        department_id: u64,
        score: i64,
    ) -> DispatchResult {
        ReputationScoreOfAccount::<T>::mutate(address, |reputation_score| {
            if let Some(reputation_score) = reputation_score.as_mut() {
                reputation_score.add_department(department_id.clone(), score);
            } else {
                *reputation_score = Some(ReputationScore::new());
                reputation_score
                    .as_mut()
                    .unwrap()
                    .add_department(department_id.clone(), score);
            }
        });
        Ok(())
    }

    pub fn update_department_reputation_score(
        address: T::AccountId,
        department_id: u64,
        score: i64,
    ) -> DispatchResult {
        ReputationScoreOfAccount::<T>::mutate(address, |reputation_score| {
            if let Some(reputation_score) = reputation_score.as_mut() {
                reputation_score.update_department(department_id.clone(), score);
            } else {
                *reputation_score = Some(ReputationScore::new());
                reputation_score
                    .as_mut()
                    .unwrap()
                    .update_department(department_id.clone(), score);
            }
        });
        Ok(())
    }

    pub fn add_reputation_score_to_department(
        address: T::AccountId,
        department_id: u64,
        amount: i64,
    ) -> DispatchResult {
        ReputationScoreOfAccount::<T>::mutate(address, |reputation_score| {
            if let Some(reputation_score) = reputation_score.as_mut() {
                reputation_score.add_score(department_id.clone(), amount);
            } else {
                *reputation_score = Some(ReputationScore::new());
                reputation_score
                    .as_mut()
                    .unwrap()
                    .add_score(department_id.clone(), amount);
            }
        });
        Ok(())
    }

    pub fn get_department_reputation_score(
        address: T::AccountId,
        department_id: u64,
    ) -> Option<i64> {
        ReputationScoreOfAccount::<T>::get(address).and_then(|reputation_score| {
            reputation_score.get_department_score(department_id.clone())
        })
    }

    pub fn get_all_department_reputation_scores(address: T::AccountId) -> Vec<(u64, i64)> {
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

impl<T: Config> Pallet<T> {
    pub fn is_member_in_group_district(
        group_id: u64,
        member: T::AccountId,
    ) -> Result<bool, DispatchError> {
        let group = Groups::<T>::get(group_id).ok_or(Error::<T>::GroupNotFound)?;

        for dept_id in group.district_departments.iter() {
            let members = DepartmentMembers::<T>::get(dept_id);
            if members.contains(&member) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_member_in_group_specialization(
        group_id: u64,
        member: T::AccountId,
    ) -> Result<bool, DispatchError> {
        let group = Groups::<T>::get(group_id).ok_or(Error::<T>::GroupNotFound)?;

        for dept_id in group.specialization_departments.iter() {
            let members = DepartmentMembers::<T>::get(dept_id);
            if members.contains(&member) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_member_and_score_in_group_specialization(
        group_id: u64,
        member: T::AccountId,
    ) -> Result<(bool, i64), DispatchError> {
        let group = Groups::<T>::get(group_id).ok_or(Error::<T>::GroupNotFound)?;

        for dept_id in group.specialization_departments.iter() {
            let members = DepartmentMembers::<T>::get(dept_id);
            if members.contains(&member) {
                let score = ReputationScoreOfAccount::<T>::get(member)
                    .and_then(|reputation_score| {
                        reputation_score.get_department_score(dept_id.clone())
                    })
                    .unwrap_or_default();
                return Ok((true, score));
            }
        }

        Ok((false, 0))
    }

    pub fn are_district_departments_empty(group_id: u64) -> Result<bool, DispatchError> {
        let group = Groups::<T>::get(group_id).ok_or(Error::<T>::GroupNotFound)?;

        Ok(group.district_departments.is_empty())
    }

    pub fn are_specialization_departments_empty(group_id: u64) -> Result<bool, DispatchError> {
        let group = Groups::<T>::get(group_id).ok_or(Error::<T>::GroupNotFound)?;

        Ok(group.specialization_departments.is_empty())
    }

    pub fn is_member_in_group_district_and_specialization(
        group_id: u64,
        member: T::AccountId,
    ) -> Result<bool, DispatchError> {
        let group = Groups::<T>::get(group_id).ok_or(Error::<T>::GroupNotFound)?;

        let has_districts = !group.district_departments.is_empty();
        let has_specializations = !group.specialization_departments.is_empty();

        // Case: No districts and no specializations
        if !has_districts && !has_specializations {
            return Err(Error::<T>::GroupHasNoDepartments.into());
        }

        let mut in_district = false;
        let mut in_specialization = false;

        // Check district membership if districts exist
        if has_districts {
            for dept_id in &group.district_departments {
                let members = DepartmentMembers::<T>::get(dept_id);
                if members.contains(&member) {
                    in_district = true;
                    break;
                }
            }
        }

        // Check specialization membership if specializations exist
        if has_specializations {
            for dept_id in &group.specialization_departments {
                let members = DepartmentMembers::<T>::get(dept_id);
                if members.contains(&member) {
                    in_specialization = true;
                    break;
                }
            }
        }

        // Fallback logic based on emptiness
        if !has_districts {
            return Ok(in_specialization);
        }
        if !has_specializations {
            return Ok(in_district);
        }

        // Both have departments, so require presence in both
        Ok(in_district && in_specialization)
    }
}
