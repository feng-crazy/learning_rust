fn pointer(){
    let a = [1,2,3];
    let b = &a;
    println!("a:{:p}", &a); // 0x7ffcbc067704
    println!("b:{:p}", b); // 0x7ffcbc067704

// 要获取可变引用，必须先声明可变绑定
    let mut c = vec![1, 2, 3];
    // c.push(4);
// 通过 &mut 得到可变引用
    let d = &mut c;
    // d = &mut vec![1, 2, 4];
    d.push(4);
    println!("{:?}", d); // [1, 2, 3, 4]
    let e = &42;
    assert_eq!(42, *e);
}


#[cfg(test)]
mod tests {
    use crate::pointer::pointer;

    #[test]
    fn it_works() {
        pointer()
    }
}