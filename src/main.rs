// Dem Notes
mod variables;
mod compound_types;
mod iterators;
mod ownership;
mod slicing;
mod structs;
mod enums;
mod funcs;
mod collections;

use crate::ownership::owner_notes;
use crate::slicing::slice_notes;
use crate::collections::col_notes;

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

    // demonstrates return syntax for rust
    // let check = funcs::another_function(5);
    // println!("result from another_function: {}", check);
}



