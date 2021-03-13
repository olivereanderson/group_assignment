//! This module introduces the concept of an assigner
//! A type implementing the assigner trait may assign subjects to groups
pub mod first_come_first_served;
pub mod propose_and_reject;
use crate::groups::Group;
use crate::subjects::Subject;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
/// Error indicating that it is not possible to assign the given subjects to groups under the current capacity constraints.
pub struct TotalCapacityError {}
impl fmt::Display for TotalCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insufficient capacity: The combined group capacity is less than the number of subjects")
    }
}
pub trait Assigner {
    /// Assign the given subjects to the given groups
    /// When the total capacity of the groups is sufficient a pair of maps (subject ids -> group ids, group ids -> subject ids) is returned
    fn assign<S: Subject, G: Group>(
        subjects: &Vec<S>,
        groups: &Vec<G>,
    ) -> Result<(HashMap<u64, u64>, HashMap<u64, Vec<u64>>), TotalCapacityError>;

    /// This method must be called by assign and in the case of an error it must be forwarded.
    fn sufficient_capacity<S: Subject, G: Group>(
        subjects: &Vec<S>,
        groups: &Vec<G>,
    ) -> Result<(), TotalCapacityError> {
        let capacity: i32 = groups.iter().map(|x| x.capacity()).sum();
        if capacity >= (subjects.len() as i32) {
            Ok(())
        } else {
            Err(TotalCapacityError {})
        }
    }
}

/// Trait enabling group membership management.
/// The assigners in this library will typically use types implementing this trait.
trait GroupManager: Group {
    /// A many to one mapping from the ids of the managed group's subjects to the group's id
    fn subjects_ids_to_group_id(&self) -> HashMap<u64, u64>;

    /// A one to many mapping from the managed group's id to the ids of its subjects
    fn group_id_to_subject_ids(&self) -> HashMap<u64, Vec<u64>>;

    /// Indicates whether the managed group is full
    fn full(&self) -> bool;
}

/// Transforms a vector of group managers into a pair of mappings representing group assignments.
/// The first of these mappings takes subjects ids to the id of their assigned group.
/// The second mapping takes an id of a group and returns a vector of the ids of the subjects assigned to this group.
fn assign_from_group_managers<M: GroupManager>(
    mut group_managers: Vec<M>,
) -> (HashMap<u64, u64>, HashMap<u64, Vec<u64>>) {
    let (init_subjects_mapper, init_groups_mapper): (HashMap<u64, u64>, HashMap<u64, Vec<u64>>) =
        group_managers
            .pop()
            .map(|x| (x.subjects_ids_to_group_id(), x.group_id_to_subject_ids()))
            .unwrap_or((HashMap::new(), HashMap::new()));

    let (subject_identifiers_to_group_identifiers, group_identifiers_to_subject_identifiers): (
        HashMap<u64, u64>,
        HashMap<u64, Vec<u64>>,
    ) = group_managers
        .iter()
        .map(|x| (x.subjects_ids_to_group_id(), x.group_id_to_subject_ids()))
        .fold((init_subjects_mapper, init_groups_mapper), |mut acc, x| {
            acc.0.extend(x.0);
            acc.1.extend(x.1);
            acc
        });

    (
        subject_identifiers_to_group_identifiers,
        group_identifiers_to_subject_identifiers,
    )
}

/// The assigners in this library will typically use some decoration of this data structure to provide assignments
struct SimpleGroupManager<'a, S, G>
where
    S: Subject,
    G: Group,
{
    group: &'a G,
    subjects: Vec<&'a S>, // members to be assigned to the corresponding group
}

impl<'a, S: Subject, G: Group> Group for SimpleGroupManager<'a, S, G> {
    fn id(&self) -> u64 {
        self.group.id()
    }

    fn capacity(&self) -> i32 {
        self.group.capacity()
    }
}

impl<'a, S: Subject, G: Group> SimpleGroupManager<'a, S, G> {
    fn new(group: &'a G, subjects: Vec<&'a S>) -> Self {
        Self { group, subjects }
    }
}

impl<'a, S: Subject, G: Group> GroupManager for SimpleGroupManager<'a, S, G> {
    fn full(&self) -> bool {
        self.subjects.len() as i32 >= self.capacity()
    }

    fn subjects_ids_to_group_id(&self) -> HashMap<u64, u64> {
        let id = self.id();
        let map: HashMap<u64, u64> = self.subjects.iter().map(|x| (x.id(), id)).collect();
        map
    }

    fn group_id_to_subject_ids(&self) -> HashMap<u64, Vec<u64>> {
        let id = self.id();
        let subject_ids: Vec<u64> = self.subjects.iter().map(|x| x.id()).collect();
        let mut map: HashMap<u64, Vec<u64>> = HashMap::new();
        map.insert(id, subject_ids);
        map
    }
}
