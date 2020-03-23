// Dem Notes
mod collections;
mod compound_types;
mod enums;
mod error_handling;
mod funcs;
mod generics;
mod iterators;
mod ownership;
mod slicing;
mod structs;
mod traits;
mod variables;

use crate::collections::col_notes;
use crate::ownership::owner_notes;
use crate::slicing::slice_notes;

// Brings an external crate into scope
// extern crate restaurant;

fn main() {
    variables::variable();
    compound_types::compound_types();
    iterators::iterator();
    owner_notes::ownership();
    slice_notes::example();
    structs::struct_exploration();
    enums::enum_stuff();
    col_notes::vectors();
    col_notes::strings();
    col_notes::hash_map();
    match error_handling::read_username_from_file() {
        Ok(_) => println!("everything is ok"),
        Err(e) => panic!("unable to read from file: {}", e),
    };

    error_handling::parsing_notes();
    error_handling::struct_handling();

    // demonstrates return syntax for rust
    let check = funcs::another_function(5);
    println!("result from another_function: {}", check);

    let target_arr: [i32; 5] = [1, 22, 55, 33, 32];
    let largest = generics::largest(&target_arr);
    println!("Largest value of {:?} is {}", target_arr, largest);

    let largest_generic = generics::generic_largest(&target_arr);
    println!(
        "Largest value of {:?} derived from Generics is {}",
        target_arr, largest_generic
    );

    let returned_t = generics::get_t(&target_arr);
    println!("Largest value of {:?} is {}", target_arr, returned_t);

    traits::trait_notes();
}
