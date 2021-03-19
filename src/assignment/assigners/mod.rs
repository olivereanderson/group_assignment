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
//! ## The propose and reject algorithm in detail:
//! [ProposeAndReject] uses the following assignment algorithm: First all subjects are assigned to the group of their first choice (or more generally of lowest possible dissatisfaction rating).
//! Then if some groups become overfull as a result of this assignment, the overfull groups propose in turn to the remaining groups to accept one of their subjects.
//! A group that is proposed to will say "no" if it is full and all its current members are at least equally satisfied with their assignment compared to the subject
//! it is proposed to accept. Otherwise the group can accept the new subject, but if the proposed group is already at full capacity, it must first discard its most dissatisfied member
//! and return it to the group according to the discarded member's first choice regardless of capacity conctraints. This propose and reject/accept process continues until there are no
//! more overfull groups.   

mod first_come_first_served;
mod propose_and_reject;
use super::group_management::*;
use crate::assignment::errors::TotalCapacityError;
use crate::groups::Group;
use crate::subjects::Subject;
pub use first_come_first_served::FirstComeFirstServed;
pub use propose_and_reject::ProposeAndReject;
use std::collections::HashMap;

/// Trait enabling group assignments. 
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

//-------------------------------- Group managers -----------------------------------------------------------------------------------------------
