extern crate regex;
extern crate hello;

use regex::Regex;
use hello::hello::hello;

mod config;
mod modules;
mod data_type_test;
mod json_test;
#[macro_use]

fn main() {
    use regex::Regex;
    use hello::hello;
    use crate::config::printf_config;

    // hello::hello();
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("204-01-01"));

    hello::hello();
    hello::hello_test();
    sub_test();
    printf_config();
}


fn sub_test(){
    modules::subm::sub();
}