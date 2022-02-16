pub mod pubtest;
#[cfg(test)]
mod tests {
    use crate::pubtest;

    #[test]
    fn it_works() {
        pubtest::use2::add()
    }
}
