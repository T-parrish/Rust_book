// Each file in the tests directory is a crate, so we need
// to bring the library into each crate's scope for testing
use adder;

// by putting the mod.rs file under a common directory
// cargo knows not to compile that file for testing purpose
// we can still bring its functionality into scope and test interop
mod common;

// Test directory gets special treatment from compiler
// only compiles files in this directory when running cargo test
// can call specific tests like so: cargo test --test integration_tests  
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}