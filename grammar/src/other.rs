#[cfg(test)]
mod tests {

    fn r_test() -> i32 {
        // 5
        return 6
    }
    #[test]
    fn it_works() {
        let y = {
            let x = 3;
            x + 1
        };
        println!("r_test is: {}", r_test());
        println!("The value of y is: {}", y);

        let number = 3;

        let num = if number < 5 {
            println!("condition was true");
            46
        } else {
            println!("condition was false");
            47
        };
        println!("The value of num is: {}",num);
    }

    fn where_test(){
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("LIFTOFF!!!");
        let mut s:&str = "hello";
        s = "testtt";
        println!("{}", s);
        let mut ss:String = String::from("hello");
        // ss = String::from("tttt");
        ss.push_str(", II")
    }
}