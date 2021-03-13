//! # Groups
//! This module introduces the group trait

pub trait Group {
    /// The group's id
    fn id(&self) -> u64;

    /// The groups capacity
    fn capacity(&self) -> i32;
}

#[cfg(test)]
pub mod test_utils;
