// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn return_tuple() -> (i32, i32) {
    (5, 5)
}

fn main() {
    let (data_x, value_y) = return_tuple();
    if data_x > 5 && value_y > 5 {
        println!("{:?}", "False");
    } else if data_x < 5 && value_y < 5 {
        println!("{:?}", "False")
    } else if data_x == 5 && value_y == 5 {
        println!("{:?}", "True");
    } else {
        print!("Something went wrong");
    }
}
