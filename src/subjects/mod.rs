//! # Subjects
//! This module defines defining the subject trait

use std::collections::HashMap;

/// The subjects to be placed in groups must implement this trait
pub trait Subject {
    /// A measure for how displeased the subject will be after being assigned to the corresponding group
    fn dissatisfaction(&self, group_id: &u32) -> u32;

    /// Id used to identify the subject.
    /// Every binding to a type that implements the subject trait is expected to have a unique id.
    ///
    /// We do not require the images of this map and the equally named function in the [group trait](crate::groups::Group) to be disjoint.
    fn id(&self) -> u32;
}

/// A simple subject type.
pub struct DefaultSubject {
    id: u32,
    preferences: HashMap<u32, u32>,
    default_dissatisfaction: u32,
}
impl DefaultSubject {
    /// Constructor
    ///
    /// ```
    /// use group_assignment::Subject;
    /// use group_assignment::DefaultSubject;
    /// use std::collections::HashMap;
    /// let id = 42_u32;
    /// let preferences: HashMap<u32,u32> = [(1_u32,0_u32),(3,2)].iter().cloned().collect();
    /// let default_dissatisfaction = 1_u32;
    /// let subject = DefaultSubject::new(id, preferences, default_dissatisfaction);
    /// assert_eq!(id,subject.id());
    /// assert_eq!(0,subject.dissatisfaction(&1_u32));
    /// assert_eq!(2, subject.dissatisfaction(&3_u32));
    /// assert_eq!(1, subject.dissatisfaction(&1000_u32));
    /// ```
    pub fn new(id: u32, preferences: HashMap<u32, u32>, default_dissatisfaction: u32) -> Self {
        Self {
            id,
            preferences,
            default_dissatisfaction,
        }
    }
}
impl Subject for DefaultSubject {
    fn id(&self) -> u32 {
        self.id
    }

    fn dissatisfaction(&self, group_id: &u32) -> u32 {
        self.preferences
            .get(group_id)
            .copied()
            .unwrap_or(self.default_dissatisfaction)
    }
}

#[cfg(test)]
pub(crate) mod test_utils;
