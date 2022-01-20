pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Module `hosting` is private [E0603]
    // crate::front_of_house::hosting::hosting::fn_prova();
}