// Define a module with the 'mod' keyword
// Not only useful for organizing code, it also defines a privacy boundary
// If you want to make something private, put it in a module
// Items in a parent module can’t use the private items inside child modules, 
// but items in child modules can use the items in their ancestor modules
// the following syntax imports children from front_of_house.rs and back_of_house.rs
mod front_of_house;
mod back_of_house;

fn serve_order() {}


// Bring hosting module into scope so we can reference it as 'hosting::'
// rather than referring to it as 'crate::front_of_house::hosting::'
// Idiomatic to bring path up to module into scope so it is clear that the 
// associated function / method is not local. For other items, (eg enum, struct)
// it is idiomatic to bring the full path into scope.
// Bringing a module into scope makes it private, but you can re-export it
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path using crate root, similar to using '/' in a shell environment
    // Need hosting and add_to_waitlist function to both be public
    hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Soooup: {:?}, salaaaad: {:?}",order1, order2 )
}

// when bringing two items with the same name into scope
// this is how you refer to them. Otherwise there would be a name
// collision when trying to figure out whether to use 
// std::fmt::Result or std::io::Result

// use std::fmt; 
// use std::io;

// fn function1() -> fmt::Result (makes sure to use fmt::Result Type)
// fn function2() -> io::Result<()> (makes sure to use io::Result Type)

// can also specify and use aliases
// use std::io::Result as io_result;
// fn function3() -> io_result<()>

// use std::{cmp::Ordering, io}; same as: 
// use std::cmp::Ordering 
// use std::cmp::io


// use std::io::{self, Write}; same as:
// use std::io;
// use std::io::Write;

// Glob operator pulls all public items defined in a path into current scope
// use std::collections::*;

