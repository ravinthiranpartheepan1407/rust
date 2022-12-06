// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Blue,
    Black,
}

fn pick(choose: Colors) {
    match choose {
        Colors::Blue => println!("Blue color"),
        Colors::Black => println!("Black Color"),
    }
}

fn main() {
    pick(Colors::Blue);
}
