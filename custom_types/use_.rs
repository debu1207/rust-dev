#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // manual scoping
    use crate::Stage::{Beginner, Advanced};
    // automatically use each name inside 'Role'
    use crate::Role::*;

    let stage = Beginner;
    let role = Student;

    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        Student => println!("Students are acquireing knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}