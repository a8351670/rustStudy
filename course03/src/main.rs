use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    id: u32,
    name: String,
    clubs: Vec<u32>,
    classes: Vec<u32>,
    courses: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Club {
    id: u32,
    name: String,
    members: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Class {
    id: u32,
    name: String,
    students: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Course {
    id: u32,
    name: String,
    students: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    clubs: HashMap<u32, Club>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
}

impl StudentManagementSystem {
    fn new() -> Self {
        StudentManagementSystem {
            students: HashMap::new(),
            clubs: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
        }
    }

    // 学生管理
    fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students.get(&student_id)
    }

    fn remove_student(&mut self, student_id: u32) -> Option<Student> {
        self.students.remove(&student_id)
    }

    // 社团管理
    fn add_club(&mut self, club: Club) {
        self.clubs.insert(club.id, club);
    }

    fn get_club(&self, club_id: u32) -> Option<&Club> {
        self.clubs.get(&club_id)
    }

    fn remove_club(&mut self, club_id: u32) -> Option<Club> {
        self.clubs.remove(&club_id)
    }

    // 班级管理
    fn add_class(&mut self, class: Class) {
        self.classes.insert(class.id, class);
    }

    fn get_class(&self, class_id: u32) -> Option<&Class> {
        self.classes.get(&class_id)
    }

    fn remove_class(&mut self, class_id: u32) -> Option<Class> {
        self.classes.remove(&class_id)
    }

    // 课程管理
    fn add_course(&mut self, course: Course) {
        self.courses.insert(course.id, course);
    }

    fn get_course(&self, course_id: u32) -> Option<&Course> {
        self.courses.get(&course_id)
    }

    fn remove_course(&mut self, course_id: u32) -> Option<Course> {
        self.courses.remove(&course_id)
    }
}

fn main() {
    let mut system = StudentManagementSystem::new();

    // 添加学生
    let student1 = Student {
        id: 1,
        name: "Alice".to_string(),
        clubs: vec![1, 2],
        classes: vec![1],
        courses: vec![1, 2],
    };

    let student2 = Student {
        id: 2,
        name: "Bob".to_string(),
        clubs: vec![1],
        classes: vec![2],
        courses: vec![1, 3],
    };

    system.add_student(student1);
    system.add_student(student2);

    // 添加社团
    let club1 = Club {
        id: 1,
        name: "Chess Club".to_string(),
        members: vec![1, 2],
    };

    let club2 = Club {
        id: 2,
        name: "Music Club".to_string(),
        members: vec![1],
    };

    system.add_club(club1);
    system.add_club(club2);

    // 添加班级
    let class1 = Class {
        id: 1,
        name: "class1".to_string(),
        students: vec![1],
    };

    // 添加班级
    let class2 = Class {
        id: 2,
        name: "class2".to_string(),
        students: vec![2],
    };
    system.add_class(class1);
    system.add_class(class2);
}