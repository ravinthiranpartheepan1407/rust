// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

#[derive(Debug)]
struct Customer {
    age: Option<i32>,
}

fn restrict(value: i32) -> Result<Customer, String> {
    // let values = Customer { age: Some(2) };
    let value = Customer { age: Some(2) };
    if value.age == Some(2) {
        Ok(Customer::age(2))
    } else {
        Err("Cannot make the purchase".to_owned())
    }
}

fn main() {
    let purchase = restrict(2);
    match purchase {
        Ok(_) => println!("Purchase option available"),
        Err(e) => println!("Pruchase option locked {:?}", e),
    }
}

#[derive(Debug)]

enum MainChoice {
    MainMenu,
    StartMenu,
    ExitMenu,
}

fn get_choice(input: &str) -> Result<MainChoice, String> {
    match input {
        "mainmenu" => Ok(MainChoice::MainMenu),
        "startmenu" => Ok(MainChoice::StartMenu),
        "exitmenu" => Ok(MainChoice::ExitMenu),
        _ => Err("Menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MainChoice) {
    println!("Choosen Menu: {:?}", choice);
}

fn main() {
    let choice = get_choice("mainmenu");
    match choice {
        Ok(selectedchoice) => print_choice(&selectedchoice),
        Err(e) => println!("Error {:?}", e),
    }
}
