**group_assignment: A simple library crate for preference based group assignments**

group_assignment is available under either the Apache-2.0 or the MIT license. 

# A simple example: 
```rust
use group_assignment::Subject;
use group_assignment::Group;
use group_assignment::assigners::{Assigner,FirstComeFirstServed};
use std::collections::HashMap;

struct Student {
    id: u64,
    name: String,
    preferences: HashMap<u64,i32>,
}
impl Student {
    fn new(id:u64, name:String, preferences: HashMap<u64,i32>) -> Self {
        Self{id, name, preferences}
    }
}
impl Subject for Student {
    fn id(&self) -> u64 {
        self.id
    }
    fn dissatisfaction(&self, group_id: &u64) -> i32 {
        self.preferences[group_id]
    }
}
struct GermanClass {
    id: u64,
    description: String,
    capacity: i32,
}
impl GermanClass {
    fn new(id: u64, description: String, capacity: i32) -> Self {
        Self{id,description,capacity: capacity}
    }
}
impl Group for GermanClass {
    fn id(&self) -> u64 {
        self.id
    }
    fn capacity(&self) -> i32 {
        self.capacity
    }
}

let grp_id_by_description: HashMap<String, u64> = [
    ("Early class".to_string(),101),
    ("Afternoon class".to_string(), 102)
].iter().cloned().collect();

let groups = vec![
    GermanClass::new(grp_id_by_description["Early class"], "Early class".to_string(),2),
    GermanClass::new(grp_id_by_description["Afternoon class"], "Afternoon class".to_string(), 2)
];
let prefer_early_class: HashMap<u64,i32> = [
    (grp_id_by_description["Early class"],0), (grp_id_by_description["Afternoon class"],1)
].iter().cloned().collect();  
let prefer_late_class: HashMap<u64,i32> = [
    (grp_id_by_description["Early class"],1),
    (grp_id_by_description["Afternoon class"],0)
].iter().cloned().collect();

let student_ids = [1_u64,2,3,4];
let students = vec![
    Student::new(student_ids[0], "Pansela".to_string(), prefer_early_class.clone()),
    Student::new(student_ids[1], "Kjetil".to_string(), prefer_late_class),
    Student::new(student_ids[2], "Mihaela".to_string(), prefer_early_class.clone()),
    Student::new(student_ids[3], "Ellinor".to_string(), prefer_early_class)    
];
let (student_ids_to_group_ids, group_ids_to_students_ids) =
    FirstComeFirstServed::assign(&students, &groups).unwrap();
//First student should be assigned to their first choice.
assert_eq!(grp_id_by_description["Early class"],student_ids_to_group_ids[&student_ids[0]]);
 //Now assert that the afternoon class consists of the second and fourth student
assert!(
    group_ids_to_students_ids[&grp_id_by_description["Afternoon class"]].contains(&student_ids[1])
);  
assert!(
    group_ids_to_students_ids[&grp_id_by_description["Afternoon class"]].contains(&student_ids[3])
);
 ```
