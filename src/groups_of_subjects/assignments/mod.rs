mod first_come_first_served; 
use super::Group;
use super::Subject;
use std::fmt;


#[derive(Debug, Clone)]
pub struct InsufficientCapacityError {}
impl fmt::Display for InsufficientCapacityError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insufficient capacity: The combined group capacity is less than the number of subjects")
    }
}
pub trait Assigner{
    /// Assign the given subjects to the given groups 
    fn assign(subjects: Vec<Subject>, groups: Vec<Group>) -> Result<Vec<Group>,InsufficientCapacityError>;

    /// This method must be called by assign and in the case of an error it must be forwarded. 
    fn sufficient_capacity(subjects: &Vec<Subject>, groups: &Vec<Group>) -> Result<(),InsufficientCapacityError> {
        let capacity: i32 = groups.iter().map(|x| x.capacity()).sum(); 
        if capacity >= (subjects.len() as i32) {
            Ok(())
        } else {
            Err(InsufficientCapacityError {})
        }
    }

}
