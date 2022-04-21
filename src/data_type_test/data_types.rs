#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::range_test();
    }
}

fn range_test() {
    // (1..5)是结构体std::ops::Range的一个实例
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 });
    // (1..=5)是结构体std::ops::RangeInclusive的一个实例
    assert_eq!(1..=5, RangeInclusive::new(1, 5));
    // 自带的 sum 方法用于求和
    // assert_eq!(3 + 4 + 5, (3..6).sum());
    // assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
    (3..6);

    // 每个范围都是一个迭代器，可用for 循环打印范围内的元素
    for i in 1..5 {
        println!("{}", i);
    }
    for i in 1..=5 {
        println!("{}", i);
    }
}

fn types_define_bool() {
    let _t = true;

    // 显式指定类型注解
    let _f: bool = false;

    // 用 as 转成 int
    let i: i32 = _f as i32;

    print!("{}", i);


    let x = 'r';
    let x = 'Ú';
    // 支持转义
    println!("{}", '\'');
    println!("{}", '\\');
    println!("{}", '\n');
    println!("{}", '\r');
    println!("{}", '\t');
    // 用 ASCII 码表示字符
    assert_eq!('\x2A', '*');
    assert_eq!('\x25', '%');
    // 用 unicode 表示字符
    assert_eq!('\u{CA0}', 'ಠ');
    assert_eq!('\u{151}', 'ő');
    // 可以使用 as 操作符将字符转为数字类型
    assert_eq!('%' as i8, 37);
    assert_eq!('ಠ' as i8, -96); //该字符值的高位会被截断，最终得到-96
}

fn types_define_float() {
    println!("{:?}", f32::INFINITY);
    println!("{:?}", f32::NEG_INFINITY);
    println!("{:?}", f32::NAN);
    println!("{:?}", f32::MIN);
    println!("{:?}", f32::MAX)
}

fn types_define_int() {
    // let vi8: i8 = 1;
    // let vi8: i16 = 1;
    // let vi8: i32 = 1;
    // let vi8: i64 = 1;
    // let vi8: i128 = 1;
    // let vi8: isize = 1;
    //
    //
    // let vi8: u8 = b'A';
    // let vi8: u16 = 1;
    // let vi8: u32 = 98_222;
    // let vi8: i64 = 0xff;
    // let vi8: u128 = 0o77;
    // let vi8: usize = 0b1111_0000;
}
