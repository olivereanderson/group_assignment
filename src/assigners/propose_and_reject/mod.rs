use std::collections::{HashMap, HashSet};

use super::Assigner;
use super::GroupManager;
use super::TotalCapacityError;
mod proposals;
use crate::groups::Group;
use crate::subjects::Subject; 

use proposals::ProposalHandlingGroupManager;
pub struct ProposeAndReject {}

impl Assigner for ProposeAndReject {
    fn assign<S: Subject, G: Group>(
        subjects: &Vec<S>,
        groups: &Vec<G>,
    ) -> Result<(HashMap<u64, u64>, HashMap<u64, Vec<u64>>), TotalCapacityError> {
        Self::sufficient_capacity(subjects, groups)?;
        let group_managers = first_step(subjects, groups);
        // Partition the managers into those whose corresponding groups will be overfull, full, and available respectively
        let (full, available): (
            Vec<ProposalHandlingGroupManager<S, G>>,
            Vec<ProposalHandlingGroupManager<S, G>>,
        ) = group_managers.into_iter().partition(|x| x.full());
        let (overfull, exactly_full): (
            Vec<ProposalHandlingGroupManager<S, G>>,
            Vec<ProposalHandlingGroupManager<S, G>>,
        ) = full.into_iter().partition(|x| x.overfull());
        let mut resolved_managers: Vec<ProposalHandlingGroupManager<S, G>> = Vec::new();
        if overfull.len() as i32 == 0 as i32 {
            // Everyone will be assigned to their first choice and we are done :)
            resolved_managers = available.into_iter().chain(exactly_full).collect();
        }

        Ok(super::assign_from_group_managers(resolved_managers))
    }
}

// The first step of the propose and reject algorithm.
// Create a group manager for each group and register every subject to a group manager corresponding to the subjects preferred choice.
// In the most general case where a subject might have more than one group with dissatisfaction rating 0, the first one appearing in the groups vector is chosen.
fn first_step<'a, S: Subject, G: Group>(
    subjects: &'a Vec<S>,
    groups: &'a Vec<G>,
) -> Vec<ProposalHandlingGroupManager<'a, S, G>> {
    let mut group_managers: Vec<_> = Vec::new();
    let mut unprocessed_subjects_indices: HashSet<usize> =
        (0..subjects.len()).into_iter().collect();
    print!("\r\n unprocessed indices: {:?} \r\n ", unprocessed_subjects_indices); 
    for group in groups {
        let id = group.id();
        let subjects_processed_this_iteration: Vec<&'a S> = Vec::new();
        let indices_processed_this_iteration: HashSet<usize> = HashSet::new();
        let (indices_processed_this_iteration, subjects_that_have_this_group_as_their_first_choice) =
            unprocessed_subjects_indices
                .iter()
                .map(|i| (i, subjects.get(*i).unwrap()))
                .filter(|(i, x)| x.dissatisfaction(&id) == 0)
                .fold(
                    (
                        indices_processed_this_iteration,
                        subjects_processed_this_iteration,
                    ),
                    |mut acc, (i, x)| {
                        acc.0.insert(*i);
                        acc.1.push(x);
                        acc
                    },
                );
        group_managers.push(ProposalHandlingGroupManager::new_without_dissatisfaction(
            group,
            subjects_that_have_this_group_as_their_first_choice,
        ));
        unprocessed_subjects_indices = unprocessed_subjects_indices
            .difference(&indices_processed_this_iteration)
            .map(|i| *i)
            .collect();
    }
    group_managers
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::groups::test_utils::TestGroup;
    use crate::subjects::test_utils::TestSubject; 

    #[test]
    fn assign_complete_after_first_step() {
        // Subject ids 
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        // Group ids 
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        // Subjects 
        let first_subject = TestSubject::new(first_subject_id, vec![second_group_id]); /// wants the second group
        let second_subject = TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject = TestSubject::new(third_subject_id, vec![first_group_id]);
        let subjects= vec![first_subject,second_subject,third_subject];
        // Groups
        let first_group = TestGroup::new(first_group_id, 3);
        let second_group = TestGroup::new(second_group_id,1);
        let groups = vec![first_group, second_group];
        /// Check that the first subject is assigned to the second group
        let (subject_ids_to_group_ids, group_ids_to_subject_ids) = ProposeAndReject::assign(&subjects, &groups).unwrap();
        assert_eq!(second_group_id,subject_ids_to_group_ids[&first_subject_id]);
        assert!(group_ids_to_subject_ids[&second_group_id].contains(&first_subject_id));
        /// Check that the second subject is mapped to the first group 
        assert_eq!(first_group_id, subject_ids_to_group_ids[&second_subject_id]);
        assert!(group_ids_to_subject_ids[&first_group_id].contains(&second_subject_id));
        /// Check that the third subject is mapped to the first group 
        assert_eq!(first_group_id, subject_ids_to_group_ids[&third_subject_id]);
        assert!(group_ids_to_subject_ids[&first_group_id].contains(&third_subject_id));
    }

    #[test]
    fn assign_complete_after_first_step_only_full() {
        let subject_id = 1 as u64;
        let group_id = 101 as u64; 
        let subject = TestSubject::new(subject_id,vec![group_id]);
        let group = TestGroup::new(subject_id, 1);
        let subjects = vec![subject];
        let groups = vec![group];
        let (subject_ids_to_group_ids,group_ids_to_subject_ids) = ProposeAndReject::assign(&subjects, &groups).unwrap();
        //print!("{:?}", subject_ids_to_group_ids); 
        assert_eq!(group_id, subject_ids_to_group_ids[&subject_id]);
        assert!(group_ids_to_subject_ids[&group_id].contains(&subject_id));
    }
}
