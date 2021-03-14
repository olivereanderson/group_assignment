//! # Assignment related errors
//! Module for assignment related errors

use std::fmt;
#[derive(Debug, Clone)]
// Error indicating that a group is already full while trying to add another subject.
pub(in crate::assignment) struct CapacityError {}
impl fmt::Display for CapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Insufficient capacity: The group capacity is less than the number of subjects"
        )
    }
}

#[derive(Debug, Clone)]
/// Error indicating that it is not possible to assign the given subjects to groups under the current capacity constraints.
pub struct TotalCapacityError {}
impl fmt::Display for TotalCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insufficient capacity: The combined group capacity is less than the number of subjects")
    }
}
