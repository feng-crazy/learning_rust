#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let s = String::from("hello");  // s 进入作用域

        takes_ownership(s);             // s 的值移动到函数里 ...
        // ... 所以到这里不再有效

        let x = 5;                      // x 进入作用域

        makes_copy(x);                  // x 应该移动函数里，
        // 但 i32 是 Copy 的，

        // println!("s{:?}", s);
        println!("x{:?}", x);

        let s1 = gives_ownership();         // gives_ownership 将返回值
        // 转移给 s1

        let s2 = String::from("hello");     // s2 进入作用域

        let s3 = takes_and_gives_back(s2);  // s2 被移动到

        println!("x{:?}", s1);
        // println!("x{:?}", s2);
        println!("x{:?}", s3);

    }

    fn takes_ownership(some_string: String) { // some_string 进入作用域
        println!("{}", some_string);
    } // 这里，some_string 移出作用域并调用 `drop` 方法。
    // 占用的内存被释放

    fn makes_copy(some_integer: i32) { // some_integer 进入作用域
        println!("{}", some_integer);
    }


    fn gives_ownership() -> String {             // gives_ownership 会将
        // 返回值移动给
        // 调用它的函数

        let some_string = String::from("yours"); // some_string 进入作用域.

        some_string                              // 返回 some_string
        // 并移出给调用的函数
        //
    }

    // takes_and_gives_back 将传入字符串并返回该值
    fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
        //

        a_string  // 返回 a_string 并移出给调用的函数
    }

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}