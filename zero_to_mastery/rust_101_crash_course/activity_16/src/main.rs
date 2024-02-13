struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student_1: Student = Student {
        name: String::from("Gertje"),
        locker: None,
    };

    let student_2: Student = Student {
        name: String::from("Bertje"),
        locker: Some(32434),
    };

    println!("Student: {:?}", student_2.name);
    match student_2.locker {
        Some(number) => println!("Locker number: {:?}", number),
        None => println!("No locker"),
    }
}
