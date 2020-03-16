pub fn variable() {
    // Needs to be mutable to allow for reassignment
    // assignment does not return a value (as it does in C or Ruby)
    // Which means you can't use this syntax: let x = y = 5
    let mut x = 5;
    println!("The value for mutable variable x is: {}", x);

    // reassign the value of x to 6, works because x was defined as mutable above
    x = 6;
    println!("X has been reassigned to: {}", x);

    // recreates the variable x and assigns it the value of 2
    let x = 2;
    println!("The new value of x is: {}", x);

    // Shadows the x variable with 2x
    let mut x = x * 2;
    println!("The shadowed value of x is: {}", x);

    // reassigned shadow
    x = 12;
    println!("reassigned shadow value to: {}", x);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "summed: {} \ndifference: {} \nproduct: {} \nquotient: {} \nremainder: {}",
        sum, difference, product, quotient, remainder
    );

    // chars are the most basic primitive alphabetic type
    // uses single quotes, occupy 4 bytes of space in memory
    let char_example = '😻';

    // Strings use double quotes
    let string_example = "dat strang";

    println!(
        "Example of Rust Char: {} \nExample of Rust String: {}",
        char_example, string_example
    );
}