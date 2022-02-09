use regex::Regex;
use hello::hello;
fn main() {
    // hello::hello();
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("204-01-01"));

    hello::hello_test()
}
