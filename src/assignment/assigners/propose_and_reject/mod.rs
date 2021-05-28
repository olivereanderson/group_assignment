//! # Propose and reject
//! This module provides an [assigner](crate::assignment::assigners::Assigner) inspired by the Gale-Shapley algorithm (also known as the propose-and-reject algorithm).
//!
use std::collections::HashSet;

use super::Assigner;
use super::GroupRegistry;
use super::TotalCapacityError;
mod proposals;
use crate::subjects::Subject;
use crate::{assignment::Assignment, groups::Group};

use proposals::ProposalHandlingGroupRegistry;
/// Assigns in a manner inspired by the Gale-Shapley algorithm.
///
///
/// First all subjects are assigned to the group of their first choice (or more generally of lowest possible dissatisfaction rating).
/// Then if some groups become overfull as a result of this assignment, the overfull groups propose in turn to the remaining groups to accept one of their subjects.
///
///
/// A group that is proposed to will say "no" if it is full and all its current members are at least equally satisfied with their assignment compared to the subject
/// it is proposed to accept. Otherwise the group can accept the new subject, but if the proposed group is already at full capacity, it must first discard its most dissatisfied member
/// and return it to the group according to the discarded member's first choice regardless of capacity constraints.
/// This propose and reject/accept process continues until there are no more overfull groups.
pub struct ProposeAndReject {}

impl Assigner for ProposeAndReject {
    fn assign<S: Subject, G: Group>(
        subjects: &[S],
        groups: &[G],
    ) -> Result<Assignment, TotalCapacityError> {
        Self::sufficient_capacity(subjects, groups)?;
        let group_registries = first_step(subjects, groups);
        // Partition the managers into those whose corresponding groups will be overfull, full, and available respectively
        let (full, mut available): (
            Vec<ProposalHandlingGroupRegistry<S, G>>,
            Vec<ProposalHandlingGroupRegistry<S, G>>,
        ) = group_registries.into_iter().partition(|x| x.full());
        let (mut overfull, mut bystanders): (
            Vec<ProposalHandlingGroupRegistry<S, G>>,
            Vec<ProposalHandlingGroupRegistry<S, G>>,
        ) = full.into_iter().partition(|x| x.overfull());
        while overfull.len() > 0 {
            // The following is a workaround until destructuring assignments stabilizes: See https://github.com/rust-lang/rust/issues/71126
            let (next_overfull, next_bystanders, next_available) =
                proposal_round(overfull, bystanders, available);
            overfull = next_overfull;
            bystanders = next_bystanders;
            available = next_available;
        }
        let resolved_registries: Vec<ProposalHandlingGroupRegistry<S, G>> =
            available.into_iter().chain(bystanders).collect();

        Ok(super::assign_from_group_registries(resolved_registries))
    }
}

// The first step of the propose and reject algorithm.
// Create a group manager for each group and register every subject to a group manager corresponding to the subjects preferred choice.
// In the most general case where a subject might have more than one group with dissatisfaction rating 0, the first one appearing in the groups vector is chosen.
fn first_step<'a, S: Subject, G: Group>(
    subjects: &'a [S],
    groups: &'a [G],
) -> Vec<ProposalHandlingGroupRegistry<'a, S, G>> {
    let mut group_registries: Vec<_> = Vec::new();
    let mut unprocessed_subjects_indices: HashSet<usize> =
        (0..subjects.len()).into_iter().collect();
    for group in groups {
        let id = group.id();
        let subjects_processed_this_iteration: Vec<&'a S> = Vec::new();
        let indices_processed_this_iteration: HashSet<usize> = HashSet::new();
        let (indices_processed_this_iteration, subjects_processed_this_iteration) =
            unprocessed_subjects_indices
                .iter()
                .map(|i| (i, subjects.get(*i).unwrap()))
                .filter(|(_i, x)| x.dissatisfaction(&id) == 0)
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
        group_registries.push(ProposalHandlingGroupRegistry::new_without_dissatisfaction(
            group,
            subjects_processed_this_iteration,
        ));
        unprocessed_subjects_indices = unprocessed_subjects_indices
            .difference(&indices_processed_this_iteration)
            .map(|i| *i)
            .collect();
    }
    if unprocessed_subjects_indices.len() > 0 {
        // This means that there were subjects that gave every group a dissatisfaction rating more than 0
        // We pass these to a group manager by the first come first served principle
        group_registries = handle_subjects_without_first_choice_first_step(
            subjects,
            unprocessed_subjects_indices,
            group_registries,
        );
    }
    group_registries
}

fn handle_subjects_without_first_choice_first_step<'a, S: Subject, G: Group>(
    subjects: &'a [S],
    unprocessed_subject_indices: HashSet<usize>,
    mut group_registries: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
) -> Vec<ProposalHandlingGroupRegistry<'a, S, G>> {
    for subject in unprocessed_subject_indices
        .iter()
        .map(|i| subjects.get(*i).unwrap())
    {
        group_registries =
            super::subject_to_best_available_group_registry(subject, group_registries);
    }

    group_registries
}

fn proposal_round<'a, S: Subject, G: Group>(
    mut overfull: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    bystanders: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    mut available: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
) -> (
    Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
) {
    let mut subjects_for_reprocessing: Vec<&S> = Vec::new();
    for overfull_group in overfull.iter_mut() {
        let (transfer_destination_key, offer) = available
            .iter()
            .enumerate()
            .map(|(i, x)| (i, overfull_group.propose_transferral(x)))
            .filter(|(_i, x)| x.is_some())
            .min_by(|(_i, x), (_j, y)| x.cmp(y))
            .map(|(i, x)| (i, x.unwrap()))
            .unwrap();

        if let Some(potentially_replaced_subject) =
            overfull_group.transfer(available.get_mut(transfer_destination_key).unwrap(), offer)
        {
            subjects_for_reprocessing.push(potentially_replaced_subject);
        }
    }
    group_registries_for_next_proposal_round(
        overfull,
        bystanders,
        available,
        subjects_for_reprocessing,
    )
}

// Adds the subjects for reprocessing to the group manager of their first choice
// returns a triple consisting of the overful managers, the bystanders and the available managers repsectively
fn group_registries_for_next_proposal_round<'a, S: Subject, G: Group>(
    overfull: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    bystanders: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    available: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    subjects_for_reprocessing: Vec<&'a S>,
) -> (
    Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
) {
    let mut registries_for_update: Vec<ProposalHandlingGroupRegistry<'a, S, G>> =
        overfull.into_iter().chain(bystanders.into_iter()).collect();
    for subject in subjects_for_reprocessing {
        registries_for_update =
            subject_to_most_desired_group_registry(registries_for_update, subject);
    }
    let (overfull, bystanders): (
        Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
        Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    ) = registries_for_update
        .into_iter()
        .partition(|x| x.overfull());
    (overfull, bystanders, available)
}

fn subject_to_most_desired_group_registry<'a, S: Subject, G: Group>(
    mut proposal_registries: Vec<ProposalHandlingGroupRegistry<'a, S, G>>,
    subject: &'a S,
) -> Vec<ProposalHandlingGroupRegistry<'a, S, G>> {
    proposal_registries
        .iter_mut()
        .min_by(|x, y| {
            subject
                .dissatisfaction(&x.id())
                .cmp(&subject.dissatisfaction(&y.id()))
        })
        .map(|x| x.force_register_subject(subject));
    proposal_registries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::DefaultGroup;
    use crate::subjects::test_utils::TestSubject;
    use std::collections::HashMap;

    #[test]
    fn assign() {
        let subject_ids = [1_u32, 2, 3, 4, 5, 6, 7, 8];
        let group_ids = [101_u32, 102, 103, 104, 105];
        let preferences1 = vec![
            group_ids[0],
            group_ids[1],
            group_ids[2],
            group_ids[3],
            group_ids[4],
        ];
        let preferences2 = vec![
            group_ids[1],
            group_ids[0],
            group_ids[3],
            group_ids[2],
            group_ids[4],
        ];
        let preferences3 = vec![
            group_ids[2],
            group_ids[3],
            group_ids[4],
            group_ids[1],
            group_ids[0],
        ];
        let subjects = [
            TestSubject::new(subject_ids[0], preferences1.clone()),
            TestSubject::new(subject_ids[1], preferences1.clone()),
            TestSubject::new(subject_ids[2], preferences1.clone()),
            TestSubject::new(subject_ids[3], preferences2.clone()),
            TestSubject::new(subject_ids[4], preferences2.clone()),
            TestSubject::new(subject_ids[5], preferences3.clone()),
            TestSubject::new(subject_ids[6], preferences3.clone()),
            TestSubject::new(subject_ids[7], preferences3.clone()),
        ];
        let capacities: HashMap<u32, u32> = [
            (group_ids[0], 1),
            (group_ids[1], 1),
            (group_ids[2], 1),
            (group_ids[3], 3),
            (group_ids[4], 3),
        ]
        .iter()
        .cloned()
        .collect();
        let groups: Vec<DefaultGroup> = group_ids
            .iter()
            .map(|id| DefaultGroup::new(*id, capacities[id]))
            .collect();

        let (subject_ids_to_group_ids, group_ids_to_subjects_ids): (
            HashMap<u32, u32>,
            HashMap<u32, Vec<u32>>,
        ) = ProposeAndReject::assign(&subjects, &groups).unwrap().into();
        assert_eq!(group_ids_to_subjects_ids[&group_ids[4]].len(), 2); // Only two subjects should be assigned to the least desired gorup despite its capacity being 3
        let total_dissatisfaction: u32 = subjects
            .iter()
            .map(|x| x.dissatisfaction(&subject_ids_to_group_ids[&x.id()]))
            .sum();
        assert_eq!(total_dissatisfaction, 12);
    }

    #[test]
    fn assign_no_necessary_replacements() {
        let subject_ids = vec![1_u32, 2, 3, 4];
        let group_ids = vec![101_u32, 102, 103];
        let mut preferences: HashMap<u32, Vec<u32>> = HashMap::new();
        let preference_by_order = vec![group_ids[0], group_ids[1], group_ids[2]];
        preferences.insert(subject_ids[0], preference_by_order.clone());
        preferences.insert(subject_ids[1], preference_by_order.clone());
        preferences.insert(subject_ids[2], preference_by_order);
        preferences.insert(
            subject_ids[3],
            vec![group_ids[0], group_ids[2], group_ids[1]],
        );
        let capacities: HashMap<u32, u32> =
            [(group_ids[0], 2), (group_ids[1], 1), (group_ids[2], 1)]
                .iter()
                .cloned()
                .collect();
        let subjects: Vec<TestSubject> = subject_ids
            .iter()
            .map(|id| TestSubject::new(*id, preferences[id].clone()))
            .collect();
        let groups: Vec<DefaultGroup> = group_ids
            .iter()
            .map(|id| DefaultGroup::new(*id, capacities[id]))
            .collect();
        let (subject_ids_to_group_ids, group_ids_to_subjects_ids): (
            HashMap<u32, u32>,
            HashMap<u32, Vec<u32>>,
        ) = ProposeAndReject::assign(&subjects, &groups).unwrap().into();

        let number_of_assigned_subjects: u32 = groups
            .iter()
            .map(|x| group_ids_to_subjects_ids[&x.id()].len() as u32)
            .sum();

        assert_eq!(number_of_assigned_subjects, subject_ids.len() as u32);
        let total_dissatisfaction: u32 = subjects
            .iter()
            .map(|x| x.dissatisfaction(&subject_ids_to_group_ids[&x.id()]))
            .sum();
        assert_eq!(total_dissatisfaction, 2);
    }

    #[test]
    fn assign_complete_after_first_step() {
        let subject_ids = [1_u32, 2, 3];
        let group_ids = [101_u32, 102];
        let subjects = [
            TestSubject::new(subject_ids[0], vec![group_ids[1]]),
            TestSubject::new(subject_ids[1], vec![group_ids[0], group_ids[1]]),
            TestSubject::new(subject_ids[2], vec![group_ids[0]]),
        ];
        let groups = [
            DefaultGroup::new(group_ids[0], 3),
            DefaultGroup::new(group_ids[1], 1),
        ];
        // Check that the first subject is assigned to the second group
        let (subject_ids_to_group_ids, group_ids_to_subject_ids): (
            HashMap<u32, u32>,
            HashMap<u32, Vec<u32>>,
        ) = ProposeAndReject::assign(&subjects, &groups).unwrap().into();
        assert_eq!(group_ids[1], subject_ids_to_group_ids[&subject_ids[0]]);
        assert!(group_ids_to_subject_ids[&group_ids[1]].contains(&subject_ids[0]));
        // Check that the second subject is mapped to the first group
        assert_eq!(group_ids[0], subject_ids_to_group_ids[&subject_ids[1]]);
        assert!(group_ids_to_subject_ids[&group_ids[0]].contains(&subject_ids[1]));
        // Check that the third subject is mapped to the first group
        assert_eq!(group_ids[0], subject_ids_to_group_ids[&subject_ids[2]]);
        assert!(group_ids_to_subject_ids[&group_ids[0]].contains(&subject_ids[2]));
    }

    #[test]
    fn assign_complete_after_first_step_only_full() {
        let subject_id = 1 as u32;
        let group_id = 101 as u32;
        let subject = TestSubject::new(subject_id, vec![group_id]);
        let group = DefaultGroup::new(group_id, 1);
        let subjects = [subject];
        let groups = [group];
        let (subject_ids_to_group_ids, group_ids_to_subject_ids): (
            HashMap<u32, u32>,
            HashMap<u32, Vec<u32>>,
        ) = ProposeAndReject::assign(&subjects, &groups).unwrap().into();
        assert_eq!(group_id, subject_ids_to_group_ids[&subject_id]);
        assert!(group_ids_to_subject_ids[&group_id].contains(&subject_id));
    }

    #[test]
    fn assign_no_first_choice() {
        struct TestSubjectWithoutFirstChoice {
            id: u32,
        }
        impl Subject for TestSubjectWithoutFirstChoice {
            fn id(&self) -> u32 {
                self.id
            }
            fn dissatisfaction(&self, _group_id: &u32) -> u32 {
                1
            }
        }
        impl TestSubjectWithoutFirstChoice {
            fn new(id: u32) -> Self {
                Self { id }
            }
        }
        let first_subject_id = 1 as u32;
        let second_subject_id = 2 as u32;

        let first_subject = TestSubjectWithoutFirstChoice::new(first_subject_id);
        let second_subject = TestSubjectWithoutFirstChoice::new(second_subject_id);
        let subjects = [first_subject, second_subject];

        let first_group_id = 101 as u32;
        let second_group_id = 102 as u32;

        let first_group = DefaultGroup::new(first_group_id, 1);
        let second_group = DefaultGroup::new(second_group_id, 1);
        let groups = [first_group, second_group];
        let (_subject_ids_to_group_ids, group_ids_to_subjects_ids): (
            HashMap<u32, u32>,
            HashMap<u32, Vec<u32>>,
        ) = ProposeAndReject::assign(&subjects, &groups).unwrap().into();
        assert_eq!(1, group_ids_to_subjects_ids[&first_group_id].len() as u32);
        assert_eq!(1, group_ids_to_subjects_ids[&second_group_id].len() as u32);
    }
}
