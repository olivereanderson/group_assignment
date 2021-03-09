use crate::groups_of_subjects::Subject;

pub struct TestSubject {
    id: u64,
    preferences: Vec<u64>,
    assigned_group_id: Option<u64>
}


impl TestSubject {
    pub fn new(id: u64, preferences: Vec<u64>) -> TestSubject {
        TestSubject { id, preferences, assigned_group_id: None }
    }

}

impl Subject for TestSubject {
    fn dissatisfaction(&self, group_id: &u64) -> i32 {
        let dissatisfaction = self.preferences.iter().position(|x| x == group_id);
        dissatisfaction.unwrap_or(self.preferences.len()) as i32
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn assigned_group_id(&self) -> Option<u64> {
        self.assigned_group_id
    }

    fn update_group_membership(&mut self, new_group_id: Option<u64>) -> () {
        self.assigned_group_id = new_group_id;
    }
}