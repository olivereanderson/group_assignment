use super::Assigner;
use super::TotalCapacityError;
use crate::groups_of_subjects::Subject;
use super::TestSubject;
use super:: GroupMetadataProxy;
use crate::groups_of_subjects::GroupMetadata;
use std::{collections::{HashMap}, fmt};

impl<'a,T:Subject> GroupMetadataProxy<'a,T> {
    fn add_subject(&mut self, subject: &'a mut T) -> Result<(),FullGroupError> {
        if (self.subjects.len() as i32) < self.capacity() {
            self.subjects.push(subject);
            Ok(())
        } else {
            Err(FullGroupError{})
        }
    }
    fn is_full(&self) -> bool {
        self.subjects.len() as i32 >= self.capacity()
    }
}

struct FullGroupError {}
impl std::fmt::Debug for FullGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group is full")
    }
}

struct FirstComeFirstServed {}
impl Assigner for FirstComeFirstServed {
    fn assign<T: Subject>(subjects: &mut Vec<T>, metadata_collection: &mut Vec<GroupMetadata>) -> Result<(),TotalCapacityError> {
        Self::sufficient_capacity(subjects, metadata_collection)?;

       for subject in subjects.iter_mut() {
           if let Some( metadata) = metadata_collection.iter_mut()
           .filter(|x| !x.full())
           .min_by(|x,y| {
               subject.dissatisfaction(&x.id()).cmp(&subject.dissatisfaction(&y.id()))}) {
               metadata.add_subject_id(subject.id()).unwrap();
               subject.update_group_membership(Some(metadata.id()));
           }
       }
       Ok(()) 
    }
}


#[cfg(test)]
mod tests {
        use super::*; 
        #[test]
        fn assign() {
        // subject ids
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        let fourth_subject_id = 4 as u64; 
        // group ids
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        let third_group_id = 103 as u64;
        // subjects 
        let first_subject = TestSubject::new(first_subject_id, vec![first_group_id,third_group_id]);
        let second_subject = TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject = TestSubject::new(third_subject_id, vec![first_group_id,second_group_id]);
        let fourth_subject = TestSubject::new(fourth_subject_id, vec![second_group_id]);
        let mut subjects = vec![first_subject, second_subject, third_subject, fourth_subject];
        // groups 
        let first_group_metadata = GroupMetadata::new(first_group_id, Vec::new(),2);
        let second_group_metadata = GroupMetadata::new(second_group_id,Vec::new(),1 );
        let third_group_metadata = GroupMetadata::new(third_group_id,Vec::new(),3);
        let mut metadata_collection = vec![first_group_metadata, second_group_metadata, third_group_metadata];
        // test 
        FirstComeFirstServed::assign(&mut subjects, &mut metadata_collection).unwrap();
        // assert that the first subject is assigned to the first group 
        assert_eq!(first_group_id, subjects[0].assigned_group_id().unwrap());
        assert!(metadata_collection[0].subject_ids().contains(&first_subject_id));
        
        // assert that the second subject is assigned to the first group 
        assert_eq!(first_group_id, subjects[1].assigned_group_id().unwrap());
        assert!(metadata_collection[0].subject_ids().contains(&second_subject_id));

        // assert that the third subject is assigned to the second group 
        assert_eq!(second_group_id, subjects[2].assigned_group_id().unwrap());
        assert!(metadata_collection[1].subject_ids().contains(&third_subject_id));

        // assert that the fourth subject is assigned to the third group 
        assert_eq!(third_group_id, subjects[3].assigned_group_id().unwrap());
        assert!(metadata_collection[2].subject_ids().contains(&fourth_subject_id));
            
        }
        #[test]
        fn add_subject_group_not_full() {
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        let mut first_subject = TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let mut second_subject = TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let mut group_metadata = GroupMetadata::new(first_group_id, Vec::new(),2);
        let mut first_group_proxy = GroupMetadataProxy::new(&mut group_metadata, vec![&mut first_subject]);
        assert!(first_group_proxy.add_subject(&mut second_subject).is_ok());
    }

    #[test]
    fn add_subject_group_is_full() {
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        let mut first_subject = TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let mut second_subject = TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let mut first_group_metadata = GroupMetadata::new(first_group_id, Vec::new(),1);
        let mut first_group = GroupMetadataProxy::new(&mut first_group_metadata, vec![&mut first_subject]);
        assert!(first_group.add_subject(&mut second_subject).is_err());
    }
}