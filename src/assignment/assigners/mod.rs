//! # Assigners
//! This module introduces the concept of an assigner.
//! A type implementing the assigner trait may assign subjects to groups.
//!
//! ## Available assigners:
//! - [First come first served](FirstComeFirstServed):
//!  The subjects get assigned to their most preferred available group in turn.
//!
//! - [Propose and reject](ProposeAndReject): First assigns every subject to their first choice regardless of capacity constraints, then the overfull groups handover subjects to the not yet full groups in a manner similar to the Gale-Shapley algorithm.
//!

mod first_come_first_served;
mod propose_and_reject;
use super::{group_management::*, Assignment};
use crate::assignment::errors::TotalCapacityError;
use crate::groups::Group;
use crate::subjects::Subject;
pub use first_come_first_served::FirstComeFirstServed;
pub use propose_and_reject::ProposeAndReject;

/// Trait enabling group assignments.
pub trait Assigner {
    /// Assign the given subjects to the given groups
    /// When the total capacity of the groups is sufficient a pair of maps (subject ids -> group ids, group ids -> subject ids) is returned
    fn assign<S: Subject, G: Group>(
        subjects: &[S],
        groups: &[G],
    ) -> Result<Assignment, TotalCapacityError>;

    /// This method must be called by assign and in the case of an error it must be forwarded.
    fn sufficient_capacity<S: Subject, G: Group>(
        subjects: &[S],
        groups: &[G],
    ) -> Result<(), TotalCapacityError> {
        let capacity: i32 = groups.iter().map(|x| x.capacity()).sum();
        if capacity >= (subjects.len() as i32) {
            Ok(())
        } else {
            Err(TotalCapacityError {})
        }
    }
}
