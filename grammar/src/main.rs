mod collections;
mod pointer;
mod other;
mod owner;

fn main() {
    let mut x = 2;
    let  a = x + 1;

    println!("a = {}, x= {}",a, x);
    x = 20;
    println!("a = {}, x= {}",a, x);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {}",guess);

    crate::collections::collections();
}

