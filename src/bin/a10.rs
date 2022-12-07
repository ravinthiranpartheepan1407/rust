// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn message(gt_100: bool, lt_100: bool) {
    match gt_100 && lt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn string_check(iscaps: bool, issmall: bool) {
    match iscaps && issmall {
        true => println!("itscaps"),
        false => println!("itssmall"),
    }
}

fn main() {
    let value = 100;
    let check_gt = value > 100;
    let check_lt = value < 100;
    message(check_gt, check_lt);

    let data = "CAPS";
    let check_cap_cases = "CAPS" == data;
    let check_small_cases = "small" == data;
    string_check(check_cap_cases, check_small_cases);
}
