mod front_of_house;

// bring parent module in scope, removes the need for absolute paths
pub use crate::front_of_house::hosting;

use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::{self, Write};
// use std::io::*; // bring it all in

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // relative path
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    //
    // // Relative path
    // hosting::add_to_waitlist();

    let number: i32 = rand::thread_rng().gen_range(1, 101);

}