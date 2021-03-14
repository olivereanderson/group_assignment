//! This module introduces the concept of an assigner
//! A type implementing the assigner trait may assign subjects to groups
pub mod first_come_first_served;
pub mod propose_and_reject;
use super::group_management::*;
use crate::assignment::errors::CapacityError;
use crate::assignment::errors::TotalCapacityError;
use crate::groups::Group;
use crate::subjects::Subject;
use std::collections::HashMap;
use std::fmt;

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
