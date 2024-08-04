use super::*;

impl<T: Config> Pallet<T> {
	pub fn check_member_is_admin(who: T::AccountId, department_id: DepartmentId) -> DispatchResult {
		match <Departments<T>>::get(department_id) {
			Some(department) => {
				let admin = department.department_admin;
				ensure!(admin == who, Error::<T>::NotAdmin);
			},
			None => Err(Error::<T>::DepartmentDontExists)?,
		}
        Ok(())
	}
}
