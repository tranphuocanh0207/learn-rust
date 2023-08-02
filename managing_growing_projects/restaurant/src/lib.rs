mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),
            }
        }
    }

    fn fn_incorret_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
    let order1 = back_of_house::Appetizer::Soup;
    let mut meal = back_of_house::Breakfast::summer("chicken");
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);
}

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}