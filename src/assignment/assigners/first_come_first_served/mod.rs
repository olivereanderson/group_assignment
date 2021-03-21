// This module implements an [assigner](crate::assignment::assigners::Assigner) according to the "first come first served" principle.
use super::Assigner;
use super::GrowingGroupRegistry;
use super::SimpleGroupRegistry;
use super::TotalCapacityError;
use crate::subjects::Subject;
use crate::{assignment::Assignment, groups::Group};

/// Assigns according to the "first come first served principle"
pub struct FirstComeFirstServed {}
impl Assigner for FirstComeFirstServed {
    /// The subjects get assigned to their most preferred available group in turn.
    fn assign<S: Subject, G: Group>(
        subjects: &[S],
        groups: &[G],
    ) -> Result<Assignment, TotalCapacityError> {
        Self::sufficient_capacity(subjects, groups)?;
        let group_managers: Vec<_> = groups
            .iter()
            .map(|g| SimpleGroupRegistry::new(g, Vec::new()))
            .collect();

        let group_managers =
            subjects_to_best_available_group_registry_by_the_first_come_first_served_principle(
                subjects,
                group_managers,
            );

        Ok(super::assign_from_group_registries(group_managers))
    }
}

fn subjects_to_best_available_group_registry_by_the_first_come_first_served_principle<
    'a,
    S: Subject,
    M: GrowingGroupRegistry<'a, S>,
>(
    subjects: &'a [S],
    mut group_registries: Vec<M>,
) -> Vec<M> {
    for subject in subjects.iter() {
        group_registries =
            super::subject_to_best_available_group_registry(subject, group_registries);
    }
    group_registries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::test_utils::TestGroup;
    use crate::subjects::test_utils::TestSubject;
    use std::collections::HashMap;
    #[test]
    fn assign() {
        let subject_ids = [1_u64, 2, 3, 4];
        let group_ids = [101_u64, 102, 103];
        let subjects = [
            TestSubject::new(subject_ids[0], vec![group_ids[0], group_ids[2]]),
            TestSubject::new(subject_ids[1], vec![group_ids[0], group_ids[1]]),
            TestSubject::new(subject_ids[2], vec![group_ids[0], group_ids[1]]),
            TestSubject::new(subject_ids[3], vec![group_ids[1]]),
        ];
        let groups = vec![
            TestGroup::new(group_ids[0], 2),
            TestGroup::new(group_ids[1], 1),
            TestGroup::new(group_ids[2], 3),
        ];

        let (subject_identifiers_to_group_identifiers, group_identifiers_to_subjects_identifiers) : (HashMap<u64,u64>, HashMap<u64,Vec<u64>>) =
            FirstComeFirstServed::assign(&subjects, &groups).unwrap().into();
        // assert that the first subject is assigned to the first group
        assert_eq!(
            group_ids[0],
            subject_identifiers_to_group_identifiers[&subject_ids[0]]
        );
        assert!(group_identifiers_to_subjects_identifiers[&group_ids[0]].contains(&subject_ids[0]));

        // assert that the second subject is assigned to the first group
        assert_eq!(
            group_ids[0],
            subject_identifiers_to_group_identifiers[&subject_ids[1]]
        );
        assert!(group_identifiers_to_subjects_identifiers[&group_ids[0]].contains(&subject_ids[1]));

        // assert that the third subject is assigned to the second group
        assert_eq!(
            group_ids[1],
            subject_identifiers_to_group_identifiers[&subject_ids[2]]
        );
        assert!(group_identifiers_to_subjects_identifiers[&group_ids[1]].contains(&subject_ids[2]));

        // assert that the fourth subject is assigned to the third group
        assert_eq!(
            group_ids[2],
            subject_identifiers_to_group_identifiers[&subject_ids[3]]
        );
        assert!(group_identifiers_to_subjects_identifiers[&group_ids[2]].contains(&subject_ids[3]));
    }
}
