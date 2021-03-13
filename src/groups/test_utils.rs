//! Module providing a simple implementation of the group trait that can be used in tests
use super::Group;
pub struct TestGroup {
    id: u64,
    capacity: i32,
}
impl TestGroup {
    pub fn new(id: u64, capacity: i32) -> TestGroup {
        TestGroup { id, capacity }
    }
}

impl Group for TestGroup {
    fn id(&self) -> u64 {
        self.id
    }

    fn capacity(&self) -> i32 {
        self.capacity
    }
}
