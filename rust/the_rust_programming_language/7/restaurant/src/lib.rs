mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

//pub fn eat_at_restaurant() {
//    crate::front_of_house::hosting::add_to_waitlist();
//
//    front_of_house::hosting::add_to_waitlist();
//}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

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

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast, please", meal.toast);
    }
    // error
    // meal.seasonal_fruit = String::from("blueberries");

    // error, seasonal_fruit is a private field
    // let meal1 = back_of_house::Breakfast{ seasonal_fruit: String::from("123"), toast: String::from("456") };

    {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }

    {
        hosting::add_to_waitlist();
    }
}

mod customer {
    // error without next line, because of scope
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
