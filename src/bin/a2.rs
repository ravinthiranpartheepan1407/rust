// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(sums: i32, mul: i32) -> i32 {
    sums + mul
}

fn result(total: i32) {
    println!("{:?}", total);
}

fn main() {
    // println!("{:?} {:?}", first_name(), last_name());
    let total = sum(2, 3);
}
