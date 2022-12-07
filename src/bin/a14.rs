// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Persons {
    age: i32,
    name: String,
    color: String,
}

fn main() {
    // use owned string type in structs - String
    // &str - Borrowed sring type for passing into a function
    // Strings are automatically borrowed
    // use .to_owned() or String::from to create an owned copy of a string slice

    // let create_by_age = vec![10, 20, 30];
    let create_by_age = vec![
        Persons {
            age: 10,
            name: "a".to_owned(),
            color: "Red".to_owned(),
        },
        Persons {
            age: 15,
            name: String::from("b"),
            color: "Blue".to_owned(),
        },
    ];
    for data in &create_by_age {
        // match data {
        //     "a" => println!("{:?} {:}", data.age, data.color),
        //     _ => println!("{:?}", "No Data"),
        // }
        if data.name == "a" {
            println!("{:?} {:?}", data.age, data.color);
        } else {
            print!("No data");
        }
    }
}
