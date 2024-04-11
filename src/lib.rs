mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    
    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house;

use crate::front_of_house::hosting::add_to_waitlist; //Allows fn to be called within this scope
use crate::front_of_house::hosting::renamed_fn as RenamedFn; // 'as' allows us to rename functions
use crate::front_of_house::hosting::{clean_up_table, seat_at_table}; // Saves space on large use lists

pub fn eat_at_resaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //Seasonal fruit is private, so the following line causes an error
    //meal.seasonal_fruit = String.from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    add_to_waitlist();
    add_to_waitlist();
    RenamedFn();
    clean_up_table();
    seat_at_table();
}

fn deliver_order() {}

