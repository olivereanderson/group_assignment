use std::collections::HashMap;

use crate::{Group, Subject};
pub mod assigners;
pub mod errors;
mod group_management;

/// Describes relationships between subjects and groups
///
/// Assignments are typically obtained from an [Assigner](assigners::Assigner)
pub struct Assignment {
    subject_ids_to_group_ids: HashMap<u32, u32>,
    group_ids_to_subjects_ids: HashMap<u32, Vec<u32>>,
}
impl Assignment {
    /// Get the id of the group the given subject is assigned to.
    pub fn subject_to_group_id<S: Subject>(&self, subject: &S) -> Option<&u32> {
        self.subject_ids_to_group_ids.get(&subject.id())
    }
    /// Get the ids of the subjects assigned to the given group.
    pub fn group_to_subjects_ids<G: Group>(&self, group: &G) -> Option<&Vec<u32>> {
        self.group_ids_to_subjects_ids.get(&group.id())
    }
}

impl Default for Assignment {
    fn default() -> Self {
        let subject_ids_to_group_ids: HashMap<u32, u32> = HashMap::new();
        let group_ids_to_subjects_ids: HashMap<u32, Vec<u32>> = HashMap::new();
        Self {
            subject_ids_to_group_ids,
            group_ids_to_subjects_ids,
        }
    }
}

impl From<(HashMap<u32, u32>, HashMap<u32, Vec<u32>>)> for Assignment {
    fn from(pair_of_maps: (HashMap<u32, u32>, HashMap<u32, Vec<u32>>)) -> Self {
        Self {
            subject_ids_to_group_ids: pair_of_maps.0,
            group_ids_to_subjects_ids: pair_of_maps.1,
        }
    }
}

impl From<Assignment>
    for (
        std::collections::HashMap<u32, u32>,
        std::collections::HashMap<u32, Vec<u32>>,
    )
{
    fn from(assignment: Assignment) -> Self {
        (
            assignment.subject_ids_to_group_ids,
            assignment.group_ids_to_subjects_ids,
        )
    }
}
