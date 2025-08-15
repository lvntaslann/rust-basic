#![allow(dead_code)]

pub enum Stage{
    Beginner,
    Advanced,
}

pub enum Role{
    Student,
    Teacher,
}

pub fn secondenum(){
    let stage = Stage::Beginner;
    let role = Role::Student;

    match stage {
        Stage::Beginner => println!("Beginners are starting their learning journey"),
        Stage::Advanced => println!("Advanced learners are mastering their subjects"),
    }

    match role {
        Role::Student => println!("Students are acquiring knowledge"),
        Role::Teacher => println!("Teachers are spreading knowledge"),
    }
}