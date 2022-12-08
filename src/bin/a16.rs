// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

//Option contains any type <T> and Some data or None

struct Student {
    name: Option<String>,
    assigment: Option<i32>,
}

fn main() {
    let data = Student {
        name: Some("a".to_owned()),
        assigment: Some(12),
    };
    match data.name {
        Some(ans) => println!("{:?}", ans),
        None => print!("No answer"),
    };

    match data.assigment {
        Some(res) => print!("The result is {:?}", res),
        None => print!("No Response"),
    }
}
