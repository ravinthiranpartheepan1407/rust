// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String),
    Vip(String),
    Standard(String),
}

fn main() {
    let data = vec![
        Ticket::Backstage("Thirty".to_owned()),
        Ticket::Vip("Fourty".to_owned()),
        Ticket::Standard("Twenty".to_owned()),
    ];

    for tickets in data {
        match tickets {
            Ticket::Backstage(price) => println!("The ticket price is {:?}", price),
            Ticket::Standard(price) => println!("The ticket price is {:?}", price),
            Ticket::Vip(price) => println!("The ticket price is {:?}", price),
        }
    }
}
