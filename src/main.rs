// Dem Notes
mod variables;
mod compound_types;
mod iterators;
mod ownership;
mod slicing;
mod structs;
mod enums;
mod funcs;

use crate::ownership::owner_notes;
use crate::slicing::slice_notes;

fn main() {
    variables::variable();
    compound_types::compound_types();
    iterators::iterator();
    owner_notes::ownership();
    slice_notes::example();
    structs::struct_exploration();
    enums::enum_stuff();

    // demonstrates return syntax for rust
    let check = funcs::another_function(5);
    println!("result from another_function: {}", check);
}



