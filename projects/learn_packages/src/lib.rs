#![allow(dead_code)]
#![allow(unused_variables)]

pub mod front_of_house;
// {
    // pub mod hosting {
    //     pub fn add_to_waitlist() {println!("yes");
    
    //     super::super::serve_order()
    // }

    //     fn seat_at_table() {}
    // }

    // mod serving {
    //     fn take_order() {}

    //     // fn serve_order() {}

    //     fn take_payment() {}
    // }
// }

use crate::front_of_house::hosting;

fn serve_order () {print!("yolo")}

mod back_of_house {

    pub enum Appetizer {
    Soup,
    Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit:String,
    }

    impl Breakfast{
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()


    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

}

fn main() {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        Ok(())
    };

    fn function2() -> IoResult<()> {
        Ok(())
    };

}
