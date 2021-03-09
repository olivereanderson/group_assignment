mod proposals;
//mod assignments;

mod test_utils;
use test_utils::TestSubject; 
use std::fmt;

/// The subjects to be placed in groups must implement this trait
pub trait Subject {
    
    /// A measure for how displeased the subject will be after being assigned to the corresponding group
    fn dissatisfaction(&self, group_id: &u64) -> i32;

    /// Id used to identify the subject
    fn id(&self) -> u64;

    /// The Id of the group the subject has been assigned to
    fn assigned_group_id(&self) -> Option<u64>;

    /// Must be implemented such that update_group_membership.assigned_group_id is the identity map. 
    fn update_group_membership(&mut self, new_group_id: Option<u64>) -> ();
}

/// Structural metadata for a given group
pub struct GroupMetaData {
    id: u64,
    subject_ids: Vec<u64>,
    capacity: i32, 
} 
impl GroupMetaData {
    /// The corresponding group's id 
    pub fn id(&self) -> u64 {
        self.id
    }
    /// The ids of the corresponding group's subjects
    pub fn subject_ids(&self) -> &Vec<u64> {
        &self.subject_ids
    }
    /// The corresponding group's capacity 
    pub fn capacity(&self) -> i32 {
        self.capacity
    }
    /// True if the corresponding group is full. 
    pub fn full(&self) -> bool {
        self.subject_ids.len() as i32 >= self.capacity
    }
    pub fn new(id: u64, subject_ids: Vec<u64>, capacity: i32) -> GroupMetaData {
        GroupMetaData{id,subject_ids,capacity}
    }
    /// Used when adding subjects to the corresponding group. A CapacityError is returned if the group is already full.  
    pub fn add_subject_id(&mut self, id: u64) -> Result<(),CapacityError> {
        if self.full() {
            Err(CapacityError {})
        } else {
            self.subject_ids.push(id);
            Ok(())
        }
    }
}


#[derive(Debug, Clone)]
/// Error indicating that a group is already full while trying to add another subject.  
pub struct CapacityError {}
impl fmt::Display for CapacityError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insufficient capacity: The group capacity is less than the number of subjects")
    }
}

/// Proxy to a Groups metadata. The various assignment algorithms in this library 
/// will update group metadata through these structures. 
struct GroupMetadataProxy<'a, T=TestSubject> where T: Subject{
    metadata: &'a mut GroupMetaData,
    subjects: Vec<&'a mut T>, // members to be assigned to the corresponding group  
    highest_dissatisfaction: i32, // cached highest dissatisfaction rating obtained from the subjects field. 
}

impl<'a, T:Subject> GroupMetadataProxy<'a,T>{
    fn id(&self) -> u64 {
        self.metadata.id()
    }
    fn overfull(&self) -> bool {
        (self.subjects.len() as i32) > self.metadata.capacity()
    }

    fn new(metadata: &'a mut GroupMetaData, mut subjects: Vec<&'a mut T>) -> Self {
        let mut highest_dissatisfaction = 0;
        let id = metadata.id();
        if subjects.len() > 0 {
            subjects.sort_by(|a, b| a.dissatisfaction(&id).cmp(&b.dissatisfaction(&id)));
            highest_dissatisfaction = subjects.last().unwrap().dissatisfaction(&id);
        }
        Self{
            metadata,
            subjects,
            highest_dissatisfaction,
        }
    }
    // Should probably move this elsewhere 
    fn new_subjects_with_first_choice(metadata: &'a mut GroupMetaData, subjects: Vec<&'a mut T>) -> Self {
        Self{metadata,subjects,highest_dissatisfaction: 0 as i32}
    }

    fn highest_dissatisfaction(&self) -> i32 {
        self.highest_dissatisfaction
    }

    fn capacity(&self) -> i32 {
        self.metadata.capacity()
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn group_sorts_subjects_upon_creation() {
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        let mut first_subject = TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let mut second_subject = TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let mut metadata = GroupMetaData::new(first_group_id,Vec::new(),2);
        let first_group = GroupMetadataProxy::new(&mut metadata, vec![&mut first_subject, &mut second_subject]);
        assert_eq!(1, first_group.highest_dissatisfaction());
    }
}
