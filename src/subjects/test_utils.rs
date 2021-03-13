/// Module providing a simple type implementing the Subject trait that can be used in tests.
use crate::subjects::Subject;
#[derive(Debug)]
pub struct TestSubject {
    id: u64,
    preferences: Vec<u64>,
    assigned_group_id: Option<u64>,
}

impl TestSubject {
    pub fn new(id: u64, preferences: Vec<u64>) -> TestSubject {
        TestSubject {
            id,
            preferences,
            assigned_group_id: None,
        }
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
}
