pub mod hello;


#[cfg(test)]
mod tests {
    use crate::hello::hello;

    #[test]
    fn it_works() {
        hello()
    }
}

