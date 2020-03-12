// Notes on variables and Rust Types

fn main() {
    // Needs to be mutable to allow for reassignment
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

    // Tuples are compound Types with a fixed length,
    // they provide a way to group a number of items, maybe with different Types
    // tup = signed 32-bit int, 64-bit float, unsigned 8-bit int
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructure the tuple elements into variables
    let (x, y, z) = tup;

    println!(
        "Tuple breakdown: \
            \nSigned 32-bit int: {} \
            \n64-bit float: {} \
            \nUnsigned 8-bit int: {}",
        x, y, z
    );

    // Can also access the tuple elements with . notation
    println!(
        "1st element: {} \n2nd Element: {} \n3rd element: {}",
        tup.0, tup.1, tup.2
    );

    // Use Array when you want data on the Stack, not Heap
    // All array elements must be the same Type
    let arr_ex = [1, 2, 3, 4, 5];

    // Initializes an array of length 5 where each element is a 32-bit int
    let arr_ex_two: [i32; 5] = [1, 2, 3, 4, 5];

    // Alternate syntax for initializing an array
    // Initializes an array of length 5, full of 3's
    let arr_ex_three = [3; 5];

    println!(
        "1st element: {} 2nd element: {} 3rd element: {} \
        \n2nd ex el1: {} 2nd ex el2: {} 2nd ex el3: {} \
        \n3rd ex el1: {} 3rd ex el2: {} 3rd ex el3: {}",
        arr_ex[0],
        arr_ex[2],
        arr_ex[3],
        arr_ex_two[0],
        arr_ex_two[1],
        arr_ex_two[2],
        arr_ex_three[0],
        arr_ex_three[1],
        arr_ex_three[2]
    );

    another_function();
}


fn another_function() {
    println!("Another function.");
}