// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox {
    dimensions: i32,
    weight: i32,
    color: Colors,
}

#[derive(Debug)]
enum Colors {
    Blue,
}

// Self represents ShippingBox struct and impl

impl ShippingBox {
    fn create_box(dimensions: i32, weight: i32, color: Colors) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn show(&self) {
        println!("{:?} {:?} {:?}", self.dimensions, self.weight, self.color);
    }
}

fn main() {
    let create = ShippingBox {
        dimensions: 20,
        weight: 50,
        color: Colors::Blue,
    };

    let new = ShippingBox::create_box(create.dimensions, create.weight, create.color);
    new.show();
}
