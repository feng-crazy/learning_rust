#[derive(Debug, PartialEq)]
pub struct People {
    name: &'static str,
    gender: u32,
} // 注意这里没有分号

impl People {
    // new 方法的参数并没有 &self
    fn new(name: &'static str, gender: u32) -> Self {
        return People { name: name, gender: gender };
    }
    // 读方法，传递的是 &self 不可变引用
    fn name(&self) {
        println!("name: {:?}", self.name);
    }
    // 写方法，传递的是 &mut self 可变引用
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
    fn gender(&self) {
        let gender = if self.gender == 1 { "boy" } else { "girl" };
        println!("gender: {:?}", gender);
    }

    // 写方法，传递的是 &mut self 可变引用
    fn set_gender(&mut self, gender: u32) {
        self.gender = gender;
    }
}

#[cfg(test)]
mod tests {
    // use crate::data_type_test::structs::People;
    use super::People;
    #[test]
    fn it_works() {


        // 用 :: 来调用new方法，默认不可变
        let alex = People::new("Alex", 1);
        // 调用其他方法用 . 号，不用传递 &self
        // 为啥不直接把 &self 改成类型java的this语法呢？反正也不传递
        alex.name();
        alex.gender();
        // 也可以直接构建结构体，绕过new方法
        assert_eq!(alex, People { name: "Alex", gender: 1 });

        // 创建可变结构体
        let mut alice = People::new("Alice", 0);
        alice.name();
        alice.gender();
        assert_eq!(alice, People { name: "Alice", gender: 0 });
        // 就可以调用set方法了
        alice.set_name("Rose");
        alice.name();
        alice.set_gender(1);
        alice.gender();
        assert_eq!(alice, People { name: "Rose", gender: 1 });


        // 等价于  struct Empty {}
        struct Empty;
        let x = Empty;
        println!("{:p}", &x);
        let y = x;
        println!("{:p}", &y as *const _);
        let z = Empty;
        println!("{:p}", &z as *const _);

        // struct RangeFull;  // 标准库源码中RangeFull就是一个单元结构体
        assert_eq!((..), std::ops::RangeFull); //  RangeFull就是(..)，表示全范围
        let i:i32 = 1;
        let mut i1 =  i;
        i1 = 2;
        println!("{}", i);
        println!("{}", i1)
    }
}
