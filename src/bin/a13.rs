// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let vector = vec![10, 20, 30, 40];

    //create a borrow for iteration otherwise the ownership function will only take the 1st slice of data and terminates.

    for data in &vector {
        // if data[2] == 12 {
        //     println!("{:?}", "Thirty");
        // } else {
        //     print!("{:?}", data[2]);
        // }

        match data {
            30 => println!("{:?}", "Thirty"),
            _ => println!("{:?}", data),
        }

        // println!("{:?}", data);
    }
    println!("{:?}", vector.len());
}
