use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist as atw;
mod prova {
    pub fn fn_prova() {}
}
pub fn add_to_waitlist() {}

fn seat_at_table() {}

pub fn eat_at_restaurant() {
    let order1 = crate::back_of_house::Appetizer::Soup;
    let order2 = crate::back_of_house::Appetizer::Salad;
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // `use` for module
    hosting::add_to_waitlist();

    // `use` for objects in a module
    atw();

    prova::fn_prova();
    // Order a breakfast in the summer with Rye toast
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Field `seasonal_fruit` of struct `back_of_house::Breakfast` is private [E0451]
    // let c = crate::back_of_house::Breakfast {
    //     toast: String::from("hello"),
    //     seasonal_fruit: String::from("world")
    // };
}