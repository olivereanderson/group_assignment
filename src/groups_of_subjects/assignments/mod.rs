mod first_come_first_served; 
use super::{Group, GroupMetaData, Subject};
use super::TestSubject;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TotalCapacityError {}
impl fmt::Display for TotalCapacityError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insufficient capacity: The combined group capacity is less than the number of subjects")
    }
}
pub trait Assigner{
    /// Assign the given subjects to the given groups 
    fn assign<'a,T: Subject>(subjects: Vec<&'a mut T>, groups: Vec<&'a mut GroupMetaData>) -> Result<(),TotalCapacityError>;

    /// This method must be called by assign and in the case of an error it must be forwarded. 
    fn sufficient_capacity<T:Subject>(subjects: &Vec<&T>, groups: Vec<&GroupMetaData>) -> Result<(),TotalCapacityError> {
        let capacity: i32 = groups.iter().map(|x| x.capacity()).sum(); 
        if capacity >= (subjects.len() as i32) {
            Ok(())
        } else {
            Err(TotalCapacityError {})
        }
    }
}
