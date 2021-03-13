//! # Subjects
//! This module defines defining the subject trait

/// The subjects to be placed in groups must implement this trait
pub trait Subject {
    /// A measure for how displeased the subject will be after being assigned to the corresponding group
    fn dissatisfaction(&self, group_id: &u64) -> i32;

    /// Id used to identify the subject
    fn id(&self) -> u64;
}

#[cfg(test)]
pub mod test_utils;
