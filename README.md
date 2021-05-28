**group_assignment: A simple library crate for preference based group assignments**

 To assign members to your groups, all that needs to be done is to either use the built in default [subject](src/subjects/mod.rs#L19) and [group](src/groups/mod.rs#L15) types, or implement the [Subject](src/subjects/mod.rs#L7) and the [Group](src/groups/mod.rs#L5) traits
 for the member and group types respectively. Then an [Assigner](src/assignment/assigners/mod.rs#L22) can provide group assignments. We have thus far implemented
 two such assingers: [ProposeAndReject](src/assignment/assigners/propose_and_reject/mod.rs#L25) (inspired by the Gale-Shapley algorithm), and the simpler [FirstComeFirstServed](src/assignment/assigners/first_come_first_served/mod.rs#L10).

 # A simple example using the default subject and group types.
 ```rust 
use group_assignment::{Subject, DefaultSubject, Group, DefaultGroup};
use group_assignment::assigners::{Assigner, FirstComeFirstServed};
use std::collections::HashMap;
struct Student {
    id: u32,
    name: String,
    preferences: HashMap<u32,u32>,
}
impl Student {
    fn new(id:u32, name:String, preferences: HashMap<u32,u32>) -> Self {
        Self{id, name, preferences}
    }
}
struct GermanClass {
    id: u32,
    description: String,
    capacity: u32,
}
impl GermanClass {
    fn new(id: u32, description: String, capacity: u32) -> Self {
        Self{id,description,capacity: capacity}
    }
}
let grp_id_by_description: HashMap<String, u32> = [
    ("Early class".to_string(),101),
    ("Afternoon class".to_string(), 102)
].iter().cloned().collect();
let classes = [
    GermanClass::new(grp_id_by_description["Early class"], "Early class".to_string(),2),
    GermanClass::new(grp_id_by_description["Afternoon class"], "Afternoon class".to_string(), 2)
];
let groups: Vec<DefaultGroup> = classes.iter().map(|x| DefaultGroup::new(x.id, x.capacity)).collect();
let prefer_early_class: HashMap<u32,u32> = [
    (grp_id_by_description["Early class"],0), (grp_id_by_description["Afternoon class"],1)
].iter().cloned().collect();  
let prefer_late_class: HashMap<u32,u32> = [
    (grp_id_by_description["Early class"],1),
    (grp_id_by_description["Afternoon class"],0)
].iter().cloned().collect();
let student_ids = [1_u32,2,3,4];
let students = [
    Student::new(student_ids[0], "Pansela".to_string(), prefer_early_class.clone()),
    Student::new(student_ids[1], "Kjetil".to_string(), prefer_late_class),
    Student::new(student_ids[2], "Mihaela".to_string(), prefer_early_class.clone()),
    Student::new(student_ids[3], "Ellinor".to_string(), prefer_early_class)    
];
let subjects : Vec<DefaultSubject> = students.iter().map(|x| DefaultSubject::new(x.id, x.preferences.clone(), 2)).collect();
let assignment =
    FirstComeFirstServed::assign(&subjects, &groups).unwrap();
//First student should be assigned to their first choice.
assert_eq!(&grp_id_by_description["Early class"],assignment.subject_to_group_id(&subjects[0]).unwrap());
 //Now assert that the afternoon class consists of the second and fourth student
assert!(
    assignment.group_to_subjects_ids(&groups[1]).unwrap().contains(&student_ids[1])
);  
assert!(
    assignment.group_to_subjects_ids(&groups[1]).unwrap().contains(&student_ids[3])
);
 ```
 # A simple example where we implement the subject and group traits.  
 ```rust 
use group_assignment::{Subject, Group};
use group_assignment::assigners::{Assigner,FirstComeFirstServed};
use std::collections::HashMap;
struct Student {
    id: u32,
    name: String,
    preferences: HashMap<u32,u32>,
}
impl Student {
    fn new(id:u32, name:String, preferences: HashMap<u32,u32>) -> Self {
        Self{id, name, preferences}
    }
}
impl Subject for Student {
    fn id(&self) -> u32 {
        self.id
    }
    fn dissatisfaction(&self, group_id: &u32) -> u32 {
        self.preferences[group_id]
    }
}
struct GermanClass {
    id: u32,
    description: String,
    capacity: u32,
}
impl GermanClass {
    fn new(id: u32, description: String, capacity: u32) -> Self {
        Self{id,description,capacity: capacity}
    }
}
impl Group for GermanClass {
    fn id(&self) -> u32 {
        self.id
    }
    fn capacity(&self) -> u32 {
        self.capacity
    }
}
let grp_id_by_description: HashMap<String, u32> = [
    ("Early class".to_string(),101),
    ("Afternoon class".to_string(), 102)
].iter().cloned().collect();
let groups = [
    GermanClass::new(grp_id_by_description["Early class"], "Early class".to_string(),2),
    GermanClass::new(grp_id_by_description["Afternoon class"], "Afternoon class".to_string(), 2)
];
let prefer_early_class: HashMap<u32,u32> = [
    (grp_id_by_description["Early class"],0), (grp_id_by_description["Afternoon class"],1)
].iter().cloned().collect();  
let prefer_late_class: HashMap<u32,u32> = [
    (grp_id_by_description["Early class"],1),
    (grp_id_by_description["Afternoon class"],0)
].iter().cloned().collect();
let student_ids = [1_u32,2,3,4];
let students = [
    Student::new(student_ids[0], "Pansela".to_string(), prefer_early_class.clone()),
    Student::new(student_ids[1], "Kjetil".to_string(), prefer_late_class),
    Student::new(student_ids[2], "Mihaela".to_string(), prefer_early_class.clone()),
    Student::new(student_ids[3], "Ellinor".to_string(), prefer_early_class)    
];
let assignment =
    FirstComeFirstServed::assign(&students, &groups).unwrap();
//First student should be assigned to their first choice.
assert_eq!(&grp_id_by_description["Early class"],assignment.subject_to_group_id(&students[0]).unwrap());
 //Now assert that the afternoon class consists of the second and fourth student
assert!(
    assignment.group_to_subjects_ids(&groups[1]).unwrap().contains(&student_ids[1])
);  
assert!(
    assignment.group_to_subjects_ids(&groups[1]).unwrap().contains(&student_ids[3])
);
 ```

# License
group_assignment is available under either the Apache-2.0 or the MIT license.