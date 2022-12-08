// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Dev,
}

enum Status {
    Active,
}

struct Check {
    position: Position,
    enter: Status,
}

fn result(employees: &Check) -> Result<(), String> {
    match employees.position {
        Position::Dev => Ok(()),
        _ => Err("Restricted".to_owned()),
    };

    match employees.enter {
        Status::Active => return Ok(()),
        _ => Err("Restricted".to_owned()),
    }
}

fn print_status(employees: &Check) -> Result<(), String> {
    let data = result(employees)?;
    println!("Access Okay");
    Ok(())
}

fn main() {
    let score = Check {
        position: Position::Dev,
        enter: Status::Active,
    };
    match print_status(&score) {
        Err(e) => println!("Access Denied {:?}", e),
        _ => (),
    }
}
