use crate::mock::*;
use crate::types::{Department, DepartmentType};
use frame_support::assert_ok;

#[test]
fn test_set_department_reputation_score() {
    new_test_ext().execute_with(|| {
        let address = 1;
        let department = Department {
            name: vec![1, 2, 3],
            department_type: DepartmentType::Locality,
            id: 1,
        };
        let score = 10;

        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department.clone(),
            score
        ));

        let reputation_score = SharedStorage::reputation_score(address);
        assert!(reputation_score.is_some());
        let reputation_score = reputation_score.unwrap();
        assert_eq!(
            reputation_score.get_department_score(department.clone()),
            Some(score)
        );
    });
}

#[test]
fn test_update_department_reputation_score() {
    new_test_ext().execute_with(|| {
        let address = 1;
        let department = Department {
            name: vec![1, 2, 3],
            department_type: DepartmentType::Locality,
            id: 1,
        };
        let score = 10;

        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department.clone(),
            score
        ));

        let new_score = 20;
        assert_ok!(SharedStorage::update_department_reputation_score(
            address,
            department.clone(),
            new_score
        ));

        let reputation_score = SharedStorage::reputation_score(address);
        assert!(reputation_score.is_some());
        let reputation_score = reputation_score.unwrap();
        assert_eq!(
            reputation_score.get_department_score(department.clone()),
            Some(new_score)
        );
    });
}

#[test]
fn test_add_score_to_department() {
    new_test_ext().execute_with(|| {
        let address = 1;
        let department = Department {
            name: vec![1, 2, 3],
            department_type: DepartmentType::Locality,
            id: 1,
        };
        let score = 10;

        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department.clone(),
            score
        ));

        let amount = 5;
        assert_ok!(SharedStorage::add_reputation_score_to_department(
            address,
            department.clone(),
            amount
        ));

        let reputation_score = SharedStorage::reputation_score(address);
        assert!(reputation_score.is_some());
        let reputation_score = reputation_score.unwrap();
        assert_eq!(
            reputation_score.get_department_score(department.clone()),
            Some(score + amount)
        );
    });
}

#[test]
fn test_get_department_reputation_score() {
    new_test_ext().execute_with(|| {
        let address = 1;
        let department = Department {
            name: vec![1, 2, 3],
            department_type: DepartmentType::Locality,
            id: 1,
        };
        let score = 10;

        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department.clone(),
            score
        ));

        let department_score =
            SharedStorage::get_department_reputation_score(address, department.clone());
        assert_eq!(department_score, Some(score));
    });
}

#[test]
fn test_get_all_department_reputation_scores() {
    new_test_ext().execute_with(|| {
        let address = 1;
        let department1 = Department {
            name: vec![1, 2, 3],
            department_type: DepartmentType::Locality,
            id: 1,
        };

        let department2 = Department {
            name: vec![4, 5, 6],
            department_type: DepartmentType::Locality,
            id: 2,
        };

        let score1 = 10;
        let score2 = 20;

        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department1.clone(),
            score1
        ));
        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department2.clone(),
            score2
        ));

        let all_departments = SharedStorage::get_all_department_reputation_scores(address);
        assert_eq!(all_departments.len(), 2);
        assert!(all_departments.contains(&(department1.clone(), score1)));
        assert!(all_departments.contains(&(department2.clone(), score2)));
    });
}

#[test]
fn test_get_total_reputation_score() {
    new_test_ext().execute_with(|| {
        let address = 1;
        let department1 = Department {
            name: vec![1, 2, 3],
            department_type: DepartmentType::Locality,
            id: 1,
        };

        let department2 = Department {
            name: vec![4, 5, 6],
            department_type: DepartmentType::Locality,
            id: 2,
        };
        let score1 = 10;
        let score2 = 20;

        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department1.clone(),
            score1
        ));
        assert_ok!(SharedStorage::set_department_reputation_score(
            address,
            department2.clone(),
            score2
        ));

        let total_score = SharedStorage::get_total_reputation_score(address);
        assert_eq!(total_score, score1 + score2);
    });
}
