//! # Groups
//! This module introduces the group trait

/// The groups the subjects may choose from must implement this trait.
pub trait Group {
    /// The group's id. Every binding to a type implementing the group trait is expected to have a unique id.
    ///
    /// We do not require the images of this map and the equally named function in the [subject trait](crate::subjects::Subject) to be disjoint.  
    fn id(&self) -> u64;

    /// The groups capacity
    fn capacity(&self) -> i32;
}
/// A simple group type.
pub struct DefaultGroup {
    id: u64,
    capacity: i32,
}

impl DefaultGroup {
    pub fn new(id: u64, capacity: i32) -> Self {
        DefaultGroup { id, capacity }
    }
}

impl Group for DefaultGroup {
    fn id(&self) -> u64 {
        self.id
    }

    fn capacity(&self) -> i32 {
        self.capacity
    }
}
