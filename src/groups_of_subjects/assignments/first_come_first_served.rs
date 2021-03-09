use super::Assigner;
use super::InsufficientCapacityError;
use super::Subject;
use super:: Group;
use std::{collections::{HashMap}, fmt};

impl Group {
    fn add_subject(&mut self, subject: Subject) -> Result<(),FullGroupError> {
        if (self.subjects.len() as i32) < self.capacity {
            self.subjects.push(subject);
            Ok(())
        } else {
            Err(FullGroupError{})
        }
    }
    fn is_full(&self) -> bool {
        self.subjects.len() as i32 >= self.capacity
    }
}

struct FullGroupError {}
impl std::fmt::Debug for FullGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group is full")
    }
}

struct FirstComeFirstServed {}
impl FirstComeFirstServed {
    fn new() -> FirstComeFirstServed {
        FirstComeFirstServed{}
    }
}
 
impl Assigner for FirstComeFirstServed {
    fn assign(subjects: Vec<Subject>, groups: Vec<Group>) -> Result<Vec<Group>,InsufficientCapacityError> {
        Self::sufficient_capacity(&subjects, &groups)?;
        if groups.len() == 0 {
            Ok(groups)
        } else {
            let mut group_mapper : HashMap<_,_> = groups.into_iter().map(|x| (x.id(),x)).collect();
            for subject in subjects.into_iter() {
                group_mapper.iter_mut()
                .filter(|(_key,group)| !group.is_full())
                .min_by(|(key1,_grp1),(key2,grp2)| subject.dissatisfaction(key1).cmp(&subject.dissatisfaction(key2)))
                .map(|(key,group)| group)
                .unwrap().add_subject(subject).unwrap();
                
            }
            // the next line is a workaround until HashMap.into_values() stabilizes
            let keys: Vec<u64> = group_mapper.keys().map(|key| *key).collect();
            Ok(keys.iter().filter_map(|key| group_mapper.remove(key)).collect())
        }
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
        let first_subject = Subject::new(first_subject_id, vec![first_group_id,third_group_id]);
        let second_subject = Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject = Subject::new(third_subject_id, vec![first_group_id,second_group_id]);
        let fourth_subject = Subject::new(third_subject_id, vec![second_group_id]);
        let subjects = vec![first_subject,second_subject, third_subject, fourth_subject];
        // groups 
        let first_group = Group::new(first_group_id, Vec::new(),2);
        let second_group = Group::new(second_group_id,Vec::new(),1 );
        let third_group = Group::new(third_group_id,Vec::new(),3);
        let groups = vec![first_group, second_group, third_group];
        // test 
        let assigned_groups = FirstComeFirstServed::assign(subjects, groups).unwrap();
            
        }
        #[test]
        fn add_subject_group_not_full() {
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        let first_subject = Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject = Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let mut first_group = Group::new(first_group_id, vec![first_subject],2);
        assert!(first_group.add_subject(second_subject).is_ok());
    }

    #[test]
    fn add_subject_group_is_full() {
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        let first_subject = Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject = Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let mut first_group = Group::new(first_group_id, vec![first_subject],1);
        assert!(first_group.add_subject(second_subject).is_err());
    }
}