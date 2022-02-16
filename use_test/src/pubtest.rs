pub mod again_use{
    pub fn add(){
        println!("test...")
    }
}

mod use1{
    pub use crate::pubtest::again_use;

    pub fn add(){
        again_use::add();
    }
}

pub(crate) mod use2{
    use crate::pubtest::use1;

    pub fn add(){
        use1::again_use::add();
        use1::add()
    }
}