struct Student {
    major: String,
}

fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) -> Vec<Student> {
    for (i, student) in collection.iter_mut().enumerate() {
        let major = format!("Major {}", "Computer science");
        behavior(student, major);
    }
    collection
}

fn main() {
    let students = vec![
        Student { major: String::from("") },
        Student { major: String::from("") },
        Student { major: String::from("") },
    ];

    let updated_students = update_majors(students, assign_major);

    for (i, student) in updated_students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}
