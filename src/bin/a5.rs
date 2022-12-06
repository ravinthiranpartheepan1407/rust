// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut a = 1;
    loop {
        println!("{:?}", a);
        a = a + 1;
        if a == 4 {
            break;
        }
    }
    println!("Program completed");

    let mut b = 4;
    loop {
        println!("{:?}", b);
        b = b - 1;
        if b == 1 {
            break;
        }
    }
    println!("Program Decremented")
}
