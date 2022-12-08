// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

#[derive(Debug)]
enum Types {
    Chairs(i32),
    Beds(i32),
    Tables(i32),
    Couches(i32),
}

struct Items {
    stock: Types,
}

// fn check(item: &Items) -> Result<(), String>{
//     match item.stock{
//         Types::Chairs => Ok(()),
//         Types::Chairs => Ok(()),
//         Types::Couches => Ok(()),
//         Types::Tables => Ok(()),
//         _ => Err("Item not found".to_owned()),
//     }
// }

// fn assign(item: &Items){
//     let data = Items{
//         Types::Tables: 2,
//         Types::Beds: 3,
//         Types::Chairs: 4,
//         Types::Couches: 0,
//     };
// }

fn main() {
    let mut item = HashMap::new();

    let chair = vec![
        Items {
            stock: Types::Beds(3),
        },
        Items {
            stock: Types::Chairs(4),
        },
        Items {
            stock: Types::Tables(2),
        },
        Items {
            stock: Types::Couches(0),
        },
    ];

    let data = item.insert(1, chair);

    if data == 0 {
        println!("Out of items");
    } else {
        println!("Items Present")
    }

    println!("{:?}", data);
}
