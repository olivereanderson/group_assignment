use crate::assignment::errors::CapacityError;
use crate::groups::Group;
use crate::subjects::Subject;
use std::collections::HashMap;

use super::Assignment;
/// Trait for group membership management.
/// The assigners in this library will typically use types implementing this trait.
pub(super) trait GroupRegistry: Group {
    /// A many to one mapping from the ids of the group registry's subjects to the group's id
    fn subjects_ids_to_group_id(&self) -> HashMap<u32, u32>;

    /// A one to many mapping from the group registry's id to the ids of its subjects
    fn group_id_to_subject_ids(&self) -> HashMap<u32, Vec<u32>>;

    /// Indicates whether the managed group is full
    fn full(&self) -> bool;
}

/// Transforms a vector of group registries into a pair of mappings representing group assignments.
/// The first of these mappings takes subjects ids to the id of their assigned group.
/// The second mapping takes an id of a group and returns a vector of the ids of the subjects assigned to this group.
pub(super) fn assign_from_group_registries<M: GroupRegistry>(
    mut group_registries: Vec<M>,
) -> Assignment {
    let (init_subjects_mapper, init_groups_mapper): (HashMap<u32, u32>, HashMap<u32, Vec<u32>>) =
        group_registries
            .pop()
            .map(|x| (x.subjects_ids_to_group_id(), x.group_id_to_subject_ids()))
            .unwrap_or((HashMap::new(), HashMap::new()));

    let (subject_identifiers_to_group_identifiers, group_identifiers_to_subject_identifiers): (
        HashMap<u32, u32>,
        HashMap<u32, Vec<u32>>,
    ) = group_registries
        .iter()
        .map(|x| (x.subjects_ids_to_group_id(), x.group_id_to_subject_ids()))
        .fold((init_subjects_mapper, init_groups_mapper), |mut acc, x| {
            acc.0.extend(x.0);
            acc.1.extend(x.1);
            acc
        });

    Assignment::from((
        subject_identifiers_to_group_identifiers,
        group_identifiers_to_subject_identifiers,
    ))
}

/// Group registries with the ability to register new members
pub(super) trait GrowingGroupRegistry<'a, S>: GroupRegistry {
    fn register_subject(&mut self, subject: &'a S) -> Result<(), CapacityError>;
}

pub(super) fn subject_to_best_available_group_registry<
    'a,
    S: Subject,
    M: GrowingGroupRegistry<'a, S>,
>(
    subject: &'a S,
    mut group_registries: Vec<M>,
) -> Vec<M> {
    group_registries
        .iter_mut()
        .filter(|x| !x.full())
        .min_by(|x, y| {
            subject
                .dissatisfaction(&x.id())
                .cmp(&subject.dissatisfaction(&y.id()))
        })
        .map(|x| x.register_subject(subject).unwrap());

    group_registries
}
/// The assigners in this library will typically use some decoration of this data structure to provide assignments
#[derive(Debug)]
pub(super) struct SimpleGroupRegistry<'a, S, G>
where
    S: Subject,
    G: Group,
{
    pub(super) group: &'a G,
    pub(super) subjects: Vec<&'a S>, // members to be assigned to the corresponding group
}

impl<'a, S: Subject, G: Group> Group for SimpleGroupRegistry<'a, S, G> {
    fn id(&self) -> u32 {
        self.group.id()
    }

    fn capacity(&self) -> u32 {
        self.group.capacity()
    }
}

impl<'a, S: Subject, G: Group> SimpleGroupRegistry<'a, S, G> {
    pub(super) fn new(group: &'a G, subjects: Vec<&'a S>) -> Self {
        Self { group, subjects }
    }
}

impl<'a, S: Subject, G: Group> GroupRegistry for SimpleGroupRegistry<'a, S, G> {
    fn full(&self) -> bool {
        self.subjects.len() as u32 >= self.capacity()
    }

    fn subjects_ids_to_group_id(&self) -> HashMap<u32, u32> {
        let id = self.id();
        let map: HashMap<u32, u32> = self.subjects.iter().map(|x| (x.id(), id)).collect();
        map
    }

    fn group_id_to_subject_ids(&self) -> HashMap<u32, Vec<u32>> {
        let id = self.id();
        let subject_ids: Vec<u32> = self.subjects.iter().map(|x| x.id()).collect();
        let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
        map.insert(id, subject_ids);
        map
    }
}

impl<'a, S: Subject, G: Group> GrowingGroupRegistry<'a, S> for SimpleGroupRegistry<'a, S, G> {
    fn register_subject(&mut self, subject: &'a S) -> Result<(), CapacityError> {
        if self.full() {
            Err(CapacityError {})
        } else {
            self.subjects.push(subject);
            Ok(())
        }
    }
}
