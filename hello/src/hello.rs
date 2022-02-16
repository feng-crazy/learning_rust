use rand::Rng;
// use std::collections::HashMap;
// use std::collections::*;
// use std::time::Instant;
pub fn hello(){
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // let time = Instant::now();
    println!("secret_number {}", secret_number);
    // HashMap::new()
}

pub fn hello_test(){
    println!("Hello, test!");
}


