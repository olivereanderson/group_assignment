/// Module providing a simple type implementing the Subject trait that can be used in tests.
use crate::subjects::Subject;
#[derive(Debug)]
pub(crate) struct TestSubject {
    id: u32,
    preferences: Vec<u32>,
    assigned_group_id: Option<u32>,
}

impl TestSubject {
    pub fn new(id: u32, preferences: Vec<u32>) -> TestSubject {
        TestSubject {
            id,
            preferences,
            assigned_group_id: None,
        }
    }
}

impl Subject for TestSubject {
    fn dissatisfaction(&self, group_id: &u32) -> u32 {
        let dissatisfaction = self.preferences.iter().position(|x| x == group_id);
        dissatisfaction.unwrap_or(self.preferences.len()) as u32
    }

    fn id(&self) -> u32 {
        self.id
    }
}
