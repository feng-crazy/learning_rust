fn main() {
    annotation();
}

// rustdoc src/lib.rs --crate-name docs
// cargo doc --verbose
/// # 文档注释: Sum函数
/// 该函数为求和函数
/// # usage:
///    assert_eq!(3, sum(1, 2));
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
pub fn annotation() {
    // 这是单行注释的示例
    /*
         * 这是区块注释, 被包含的区域都会被注释
         * 你可以把/* 区块 */ 置于代码中的任何位置
    */
    /*
        注意上面区块注释中的*符号，纯粹是一种注释风格，
        实际并不需要
        */
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("2 + 3 = {}", sum(2, 3));
}
