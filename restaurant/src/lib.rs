// Modules
//
mod front_of_house;


// mod front_of_house {
//    pub mod hosting {
//       pub fn add_to_waitlist(){}

//         fn seat_at_table(){}
//     }
//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }

    fn cook_order(){}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
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


// use crate::front_of_house::hosting;

pub use crate::front_of_house::hosting;

fn serve_order() {}

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();



    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("Blueberries"); // This operation is prohibitted. Because, seasonal_fruit property is not pub
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

}


use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
// }

// fn function2() -> io::Result<()>{
// }


use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function3() -> Result {

// }
// fn function4() -> IoResult<()>{
// }


// use std::{cmp::Ordering, io};

use std::io::{self, Write};

use std::collections::*;

