//#[derive(Debug)]
mod proposals;
pub struct Subject {
    id: u64,
    preferences: Vec<u64>,
}
impl Subject {
    /// A measure for how displeased the subject will be after being assigned to the corresponding group
    pub fn dissatisfaction(&self, group_id: &u64) -> i32 {
        let dissatisfaction = self.preferences.iter().position(|x| x == group_id);
        dissatisfaction.unwrap_or(self.preferences.len()) as i32
    }
    pub fn new(id: u64, preferences: Vec<u64>) -> Subject {
        Subject { id, preferences }
    }
}
//#[derive(Debug)]
pub struct Group {
    id: u64, 
    subjects: Vec<Subject>, // the members
    capacity: i32, // The maximum number of members
    highest_dissatisfaction: i32, // the dissatisfaction rating given by the most dissatisfied member
}

impl Group {
    pub fn id(&self) -> u64 {
        self.id
    }
    fn overfull(&self) -> bool {
        (self.subjects.len() as i32) > self.capacity
    }

    pub fn new(id: u64, mut subjects: Vec<Subject>, capacity: i32) -> Group {
        let mut highest_dissatisfaction = 0;
        if subjects.len() > 0 {
            subjects.sort_by(|a, b| a.dissatisfaction(&id).cmp(&b.dissatisfaction(&id)));
            highest_dissatisfaction = subjects.last().unwrap().dissatisfaction(&id);
        }
        Group {
            id,
            subjects,
            capacity,
            highest_dissatisfaction,
        }
    }
    // Should probably move this elsewhere 
    fn new_subjects_with_first_choice(id:u64, subjects: Vec<Subject>, capacity: i32) -> Group {
        Group{id,subjects,capacity,highest_dissatisfaction: 0 as i32}
    }

    fn highest_dissatisfaction(&self) -> i32 {
        self.highest_dissatisfaction
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
        let first_subject = Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject = Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let first_group = Group::new(first_group_id, vec![first_subject, second_subject], 1);
        assert_eq!(1, first_group.highest_dissatisfaction());
    }
}
