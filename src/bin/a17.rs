// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

/// Lower to Upper Case
fn lower(a: &str) {
    let text1_result = a.to_uppercase();
    println!("Text converted to upperacse {:?}", text1_result);
}

/// Upper to Lower case
fn upper(b: &str) {
    // let text2 = "VALUE";
    let text2_result = b.to_lowercase();
    println!("Text converted to lowercase {:?}", text2_result);
}

fn main() {}
