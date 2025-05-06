#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod extras;
pub mod types;

use codec::{Decode, Encode};
use frame_support::traits::BuildGenesisConfig;
use frame_support::BoundedVec;
use frame_system::ensure_root;
use sp_std::prelude::*;
use types::ReputationScore;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type Score = i64;
type DepartmentId = u64;
use crate::types::Department;
use crate::types::DepartmentType;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type MaxDepartmentsPerGroup: Get<u32>;

        #[pallet::constant]
        type MaxMembersPerDepartment: Get<u32>;

        #[pallet::constant]
        type MaxMembersPerGroup: Get<u32>;
    }
    #[pallet::storage]
    #[pallet::getter(fn approved_citizen_address)]
    pub type ApprovedCitizenAddress<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>; // Its set, add element through binary_search

    #[pallet::storage]
    #[pallet::getter(fn approved_citizen_address_by_department)]
    pub type ApprovedCitizenAddressByDepartment<T: Config> =
        StorageMap<_, Blake2_128Concat, DepartmentId, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn positive_externality_score)]
    pub type PositiveExternalityScore<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Score, ValueQuery>;

    // Keep winning representatives of department in shared storage

    #[pallet::storage]
    #[pallet::getter(fn  reputation_score)]
    pub type ReputationScoreOfAccount<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, ReputationScore>;

    #[pallet::storage]
    pub type DepartmentCount<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    pub type Departments<T: Config> = StorageMap<_, Blake2_128Concat, u64, Department, OptionQuery>;

    #[pallet::storage]
    pub type DepartmentGroups<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        BoundedVec<u64, T::MaxDepartmentsPerGroup>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type DepartmentMembers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        BoundedBTreeSet<T::AccountId, T::MaxMembersPerDepartment>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type DepartmentGroupMembers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        BoundedBTreeSet<T::AccountId, T::MaxMembersPerGroup>,
        ValueQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub approved_citizen_address: Vec<T::AccountId>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                approved_citizen_address: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            <ApprovedCitizenAddress<T>>::put(self.approved_citizen_address.clone());
        }
    }

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored { something: u32, who: T::AccountId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        CitizenNotApproved,
        AlreadyMember,
        DepartmentAlreadyExists,
        DepartmentDoesNotExist,
        TooManyDepartmentsInGroup,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_department(
            origin: OriginFor<T>,
            name: Vec<u8>,
            department_type: DepartmentType,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let id = DepartmentCount::<T>::get();

            // Use checked_add to avoid overflow
            let next_id = id.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

            let department = Department {
                name,
                department_type,
                id,
            };

            Departments::<T>::insert(id, department);
            DepartmentCount::<T>::put(next_id);

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn create_department_group(
            origin: OriginFor<T>,
            group_id: u64,
            department_ids: Vec<u64>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            for id in &department_ids {
                ensure!(
                    Departments::<T>::contains_key(id),
                    Error::<T>::DepartmentDoesNotExist
                );
            }

            let bounded_departments: BoundedVec<u64, T::MaxDepartmentsPerGroup> =
                BoundedVec::try_from(department_ids.clone())
                    .map_err(|_| Error::<T>::TooManyDepartmentsInGroup)?;

            DepartmentGroups::<T>::insert(group_id, bounded_departments);
            // Populate members from included departments
            let mut group_members = BoundedBTreeSet::new();
            for dept_id in &department_ids {
                let members = DepartmentMembers::<T>::get(dept_id);
                for m in members {
                    group_members.try_insert(m).ok(); // Ignore overflow
                }
            }
            DepartmentGroupMembers::<T>::insert(group_id, group_members);
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn add_member_to_department(
            origin: OriginFor<T>,
            dept_id: u64,
            account: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                Departments::<T>::contains_key(dept_id),
                Error::<T>::DepartmentDoesNotExist
            );
            DepartmentMembers::<T>::mutate(dept_id, |members| {
                members.try_insert(account.clone()).ok();
            });

            // Update department group membership
            for (group_id, dept_ids) in DepartmentGroups::<T>::iter() {
                if dept_ids.contains(&dept_id) {
                    DepartmentGroupMembers::<T>::mutate(group_id, |group_members| {
                        group_members.try_insert(account.clone()).ok();
                    });
                }
            }

            Ok(())
        }
    }
}
