mod front_of_house {
    pub mod hosting {
        pub(crate) fn add_to_waitlist() {}
    }
}
mod front_of_house1 {
    use crate::front_of_house::front_of_house;

    fn hdf_test(){}
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        super::front_of_house::hosting::add_to_waitlist();
        // Relative path
        front_of_house::hosting::add_to_waitlist();
        self::hdf_test();
    }
}