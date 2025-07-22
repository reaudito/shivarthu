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
use frame_support::pallet_prelude::*;
use frame_support::traits::BuildGenesisConfig;
use frame_support::BoundedVec;
use frame_system::ensure_root;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;
use types::{Address, GpsCoordinate, ReputationScore};

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type Score = i64;
type DepartmentId = u64;
pub type MaxNameLength = ConstU32<64>;
pub type AddressNameLength = ConstU32<200>;
pub type MaxDepartmentsPerGroup = ConstU32<50>;
pub type MaxMembersPerDepartment = ConstU32<100000>;
use crate::types::DepartmentType;
use crate::types::{Department, Group};

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
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
    pub type GroupCount<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    pub type Departments<T: Config> = StorageMap<_, Blake2_128Concat, u64, Department, OptionQuery>;

    #[pallet::storage]
    pub type DepartmentMembers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        BoundedBTreeSet<T::AccountId, MaxMembersPerDepartment>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn addresses)]
    pub type Addresses<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Address, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn groups)]
    pub type Groups<T: Config> = StorageMap<_, Blake2_128Concat, u64, Group, OptionQuery>;

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
        SomethingStored {
            something: u32,
            who: T::AccountId,
        },
        MemberAddedToDepartment {
            member: T::AccountId,
            department_id: u64,
        },
        DepartmentGroupCreated {
            group_id: u64,
            departments: Vec<u64>,
        },
        AddressSaved {
            who: T::AccountId,
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
        InvalidDepartmentType,
        GroupNotFound,
        TooManyDepartments,
        GroupHasNoDepartments,
        DistrictTooLong,
        CountryTooLong,
        CityTooLong,
        InvalidLatitude,
        InvalidLongitude,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_department(
            origin: OriginFor<T>,
            name: BoundedVec<u8, MaxNameLength>,
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
        pub fn create_group(
            origin: OriginFor<T>,
            name: BoundedVec<u8, MaxNameLength>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let group_id = GroupCount::<T>::get();

            // Use checked_add to avoid overflow
            let next_id = group_id.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

            let new_group = Group {
                id: group_id,
                name,
                specialization_departments: BoundedVec::default(),
                district_departments: BoundedVec::default(),
            };

            Groups::<T>::insert(group_id, new_group);
            GroupCount::<T>::put(next_id);

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn add_specialization_department_to_group(
            origin: OriginFor<T>,
            group_id: u64,
            department_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let department =
                Departments::<T>::get(department_id).ok_or(Error::<T>::DepartmentNotFound)?;

            ensure!(
                matches!(department.department_type, DepartmentType::Specialization),
                Error::<T>::InvalidDepartmentType
            );

            Groups::<T>::try_mutate(group_id, |maybe_group| -> DispatchResult {
                let group = maybe_group.as_mut().ok_or(Error::<T>::GroupNotFound)?;
                if !group.specialization_departments.contains(&department_id) {
                    group
                        .specialization_departments
                        .try_push(department_id)
                        .map_err(|_| Error::<T>::TooManyDepartments)?;
                }
                Ok(())
            })
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn remove_specialization_department_from_group(
            origin: OriginFor<T>,
            group_id: u64,
            department_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Groups::<T>::try_mutate(group_id, |maybe_group| -> DispatchResult {
                let group = maybe_group.as_mut().ok_or(Error::<T>::GroupNotFound)?;
                group
                    .specialization_departments
                    .retain(|&id| id != department_id);
                Ok(())
            })
        }

        #[pallet::call_index(5)]
        #[pallet::weight(0)]
        pub fn add_district_department_to_group(
            origin: OriginFor<T>,
            group_id: u64,
            department_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let department =
                Departments::<T>::get(department_id).ok_or(Error::<T>::DepartmentNotFound)?;

            ensure!(
                matches!(department.department_type, DepartmentType::District),
                Error::<T>::InvalidDepartmentType
            );

            Groups::<T>::try_mutate(group_id, |maybe_group| -> DispatchResult {
                let group = maybe_group.as_mut().ok_or(Error::<T>::GroupNotFound)?;
                if !group.district_departments.contains(&department_id) {
                    group
                        .district_departments
                        .try_push(department_id)
                        .map_err(|_| Error::<T>::TooManyDepartments)?;
                }
                Ok(())
            })
        }

        #[pallet::call_index(6)]
        #[pallet::weight(0)]
        pub fn remove_district_department_from_group(
            origin: OriginFor<T>,
            group_id: u64,
            department_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Groups::<T>::try_mutate(group_id, |maybe_group| -> DispatchResult {
                let group = maybe_group.as_mut().ok_or(Error::<T>::GroupNotFound)?;
                group.district_departments.retain(|&id| id != department_id);
                Ok(())
            })
        }

        #[pallet::call_index(7)]
        #[pallet::weight(0)]
        pub fn save_address(
            origin: OriginFor<T>,
            district: Vec<u8>,
            country: Vec<u8>,
            city: Vec<u8>,
            latitude: Option<i32>,  // microdegrees
            longitude: Option<i32>, // microdegrees
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate and convert to bounded vectors
            let bounded_district = BoundedVec::<u8, AddressNameLength>::try_from(district)
                .map_err(|_| Error::<T>::DistrictTooLong)?;

            let bounded_country = BoundedVec::<u8, AddressNameLength>::try_from(country)
                .map_err(|_| Error::<T>::CountryTooLong)?;

            let bounded_city = BoundedVec::<u8, AddressNameLength>::try_from(city)
                .map_err(|_| Error::<T>::CityTooLong)?;

            // Validate coordinates if provided
            let location = if let (Some(lat), Some(lon)) = (latitude, longitude) {
                ensure!(
                    lat >= -90_000_000 && lat <= 90_000_000,
                    Error::<T>::InvalidLatitude
                );
                ensure!(
                    lon >= -180_000_000 && lon <= 180_000_000,
                    Error::<T>::InvalidLongitude
                );
                Some(GpsCoordinate {
                    latitude: lat,
                    longitude: lon,
                })
            } else {
                None
            };

            let address = Address {
                district: bounded_district,
                country: bounded_country,
                city: bounded_city,
                location,
            };

            // Store in map
            <Addresses<T>>::insert(&sender, address);

            // Emit event
            Self::deposit_event(Event::AddressSaved { who: sender });

            Ok(()) // This returns DispatchResult
        }
    }
}
