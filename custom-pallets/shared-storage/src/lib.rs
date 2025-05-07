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
    pub type DepartmentGroupCount<T: Config> = StorageValue<_, u64, ValueQuery>;

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
        MemberAddedToDepartment {
            member: T::AccountId,
            department_id: u64,
        },
        DepartmentGroupCreated {
            group_id: u64,
            departments: Vec<u64>,
        },
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
        DepartmentNotFound,
        MemberAlreadyInDepartment,
        TooManyMembers,
        GroupAlreadyExists,
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
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn add_member_to_department(
            origin: OriginFor<T>,
            department_id: u64,
            member: T::AccountId,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Ensure department exists
            ensure!(
                Departments::<T>::contains_key(&department_id),
                Error::<T>::DepartmentNotFound
            );

            DepartmentMembers::<T>::try_mutate(
                department_id,
                |members| -> Result<(), DispatchError> {
                    members
                        .try_insert(member.clone())
                        .map_err(|_| Error::<T>::MemberAlreadyInDepartment)?;
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::MemberAddedToDepartment {
                member,
                department_id,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn create_department_group(
            origin: OriginFor<T>,
            departments: BoundedVec<u64, T::MaxDepartmentsPerGroup>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let group_id = DepartmentGroupCount::<T>::get();
            let next_id = group_id.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

            ensure!(
                !DepartmentGroups::<T>::contains_key(&group_id),
                Error::<T>::GroupAlreadyExists
            );

            for dept_id in departments.iter() {
                ensure!(
                    Departments::<T>::contains_key(dept_id),
                    Error::<T>::DepartmentNotFound
                );
            }

            DepartmentGroups::<T>::insert(&group_id, departments.clone());

            DepartmentGroupCount::<T>::put(next_id);

            Self::deposit_event(Event::DepartmentGroupCreated {
                group_id,
                departments: departments.into_inner(),
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn is_member_in_department(department_id: u64, account: &T::AccountId) -> bool {
            DepartmentMembers::<T>::get(department_id).contains(account)
        }

        pub fn is_member_in_department_group(group_id: u64, account: &T::AccountId) -> bool {
            let departments = DepartmentGroups::<T>::get(group_id);

            // departments.iter()
            // Loops over each department ID in the departments collection (which is likely a Vec<u64>).

            // .all(...)
            // Returns true only if the predicate inside returns true for every item — in this case, for every department ID.
            departments
                .iter()
                .all(|dept_id| DepartmentMembers::<T>::get(*dept_id).contains(account))
        }
    }
}
