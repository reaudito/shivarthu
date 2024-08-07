use super::*;
use trait_departments::DepartmentsLink;

impl<T: Config> DepartmentsLink for Pallet<T> {
	type DepartmentId = DepartmentId;

	fn check_department_exists(department_id: DepartmentId) -> DispatchResult {
		Self::check_department_exists(department_id)
	}
}

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
	pub fn check_department_exists(department_id: DepartmentId) -> DispatchResult {
		match <Departments<T>>::get(department_id) {
			Some(_) => Ok(()),
			None => Err(Error::<T>::DepartmentDontExists)?,
		}
	}
}
