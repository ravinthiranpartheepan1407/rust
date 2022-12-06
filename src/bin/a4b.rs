// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let a = 1;
    let b = 8;
    let c = 3;

    match a {
        1 => println!("Number 1 printed"),
        2 => println!("Number 2 printed"),
        3 => println!("Number 3 printed"),
        _ => println!("Something else printed"),
    }

    match b {
        1 => println!("Number 1 printed"),
        2 => println!("Number 2 printed"),
        3 => println!("Number 3 printed"),
        _ => println!("Something else printed"),
    }

    match c {
        1 => println!("Number 1 printed"),
        2 => println!("Number 2 printed"),
        3 => println!("Number 3 printed"),
        _ => println!("Something else printed"),
    }
}
