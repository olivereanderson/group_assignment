//! # First come first served
//! This module implements an assigner according to the "first come first served" principle
use super::Assigner;
use super::GroupManager;
use super::GrowingGroupManager;
use super::SimpleGroupManager;
use super::TotalCapacityError;
use crate::groups::Group;
use crate::subjects::Subject;
use std::collections::HashMap;

pub struct FirstComeFirstServed {}
impl Assigner for FirstComeFirstServed {
    /// The subjects get assigned to their most preferred available group in turn.
    fn assign<S: Subject, G: Group>(
        subjects: &Vec<S>,
        groups: &Vec<G>,
    ) -> Result<(HashMap<u64, u64>, HashMap<u64, Vec<u64>>), TotalCapacityError> {
        Self::sufficient_capacity(subjects, groups)?;
        let group_managers: Vec<_> = groups
            .iter()
            .map(|g| SimpleGroupManager::new(g, Vec::new()))
            .collect();

        let group_managers =
            subjects_to_best_available_group_manager_by_the_first_come_first_served_principle(
                subjects,
                group_managers,
            );

        Ok(super::assign_from_group_managers(group_managers))
    }
}

fn subjects_to_best_available_group_manager_by_the_first_come_first_served_principle<
    'a,
    S: Subject,
    M: GrowingGroupManager<'a, S>,
>(
    subjects: &'a Vec<S>,
    mut group_managers: Vec<M>,
) -> Vec<M> {
    for subject in subjects.iter() {
        group_managers = super::subject_to_best_available_group_manager(subject, group_managers);
    }
    group_managers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::test_utils::TestGroup;
    use crate::subjects::test_utils::TestSubject;
    #[test]
    fn assign() {
        // subject ids
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        let fourth_subject_id = 4 as u64;
        // group ids
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        let third_group_id = 103 as u64;
        // subjects
        let first_subject =
            TestSubject::new(first_subject_id, vec![first_group_id, third_group_id]);
        let second_subject =
            TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject =
            TestSubject::new(third_subject_id, vec![first_group_id, second_group_id]);
        let fourth_subject = TestSubject::new(fourth_subject_id, vec![second_group_id]);
        let subjects = vec![first_subject, second_subject, third_subject, fourth_subject];
        // groups
        let first_group = TestGroup::new(first_group_id, 2);
        let second_group = TestGroup::new(second_group_id, 1);
        let third_group = TestGroup::new(third_group_id, 3);
        let groups = vec![first_group, second_group, third_group];
        // test
        let (subject_identifiers_to_group_identifiers, group_identifiers_to_subjects_identifiers) =
            FirstComeFirstServed::assign(&subjects, &groups).unwrap();
        // assert that the first subject is assigned to the first group
        assert_eq!(
            first_group_id,
            subject_identifiers_to_group_identifiers[&first_subject_id]
        );
        assert!(
            group_identifiers_to_subjects_identifiers[&first_group_id].contains(&first_subject_id)
        );

        // assert that the second subject is assigned to the first group
        assert_eq!(
            first_group_id,
            subject_identifiers_to_group_identifiers[&second_subject_id]
        );
        assert!(
            group_identifiers_to_subjects_identifiers[&first_group_id].contains(&second_subject_id)
        );

        // assert that the third subject is assigned to the second group
        assert_eq!(
            second_group_id,
            subject_identifiers_to_group_identifiers[&third_subject_id]
        );
        assert!(
            group_identifiers_to_subjects_identifiers[&second_group_id].contains(&third_subject_id)
        );

        // assert that the fourth subject is assigned to the third group
        assert_eq!(
            third_group_id,
            subject_identifiers_to_group_identifiers[&fourth_subject_id]
        );
        assert!(
            group_identifiers_to_subjects_identifiers[&third_group_id].contains(&fourth_subject_id)
        );
    }
}
