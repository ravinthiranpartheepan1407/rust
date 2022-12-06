// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Soda,
    // Water,
    // Orange,
}

struct Value<'a> {
    flavor: &'a str,
    ounces: f64,
}

fn pick(choose: Flavor) {
    let data = Value {
        flavor: "Sweet",
        ounces: 0.15,
    };

    match choose {
        Flavor::Soda => println!("{:?} {:?}", data.flavor, data.ounces),
    }
}

fn main() {
    pick(Flavor::Soda);
}
