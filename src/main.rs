// Dem Notes

fn main() {
    variable();
    compound_types();

    let swag = another_function(5);
    println!("result from another_function: {}", swag);

    iterators();
    ownership();

    let test_string = String::from("Booyakasha, fire pon the selectah");

    // Since slices takes a string slice, pass a slice
    let mut first_word = slices(&test_string[..]);
    println!(
        "Original sentence: {} \n1st word: {}",
        test_string, first_word
    );

    // String literals are &str type, so you can pass these by themselves
    let string_two = "Dis is de General";
    first_word = slices(string_two);
    println!(
        "Original sentence: {} \n1st word: {}",
        string_two, first_word
    );

    let user1 = build_user(
        String::from("twpearish@gmail.com"),
        String::from("JimmyJammy"),
    );

    println!(
        "{} with email address {} has logged in {} times. \nIs he currently logged in? {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    // In order to mutate the values on a struct, the entire instance must be mutable
    let mut user2 = User {
        email: String::from("sp4zium@gmail.com"),
        username: String::from("marklar"),
        active: true,
        sign_in_count: 1,
    };
    println!("User2 email before mutating: {}", user2.email);

    user2.email = String::from("wakawaka@gmail.com");
    println!("User2 email after mutating: {}", user2.email);

    // you can spread another struct's values into a struct
    // this example fills the active and sign_in_count values with
    // the values from user1
    let user3 = User {
        email: String::from("blerch@gmail.com"),
        username: String::from("wakawaka"),
        ..user1
    };

    println!(
        "email: {} username: {} sign in count: {} active: {} ",
        user3.email, user3.username, user3.sign_in_count, user3.active
    );
}

fn variable() {
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

fn compound_types() {
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
}

// Functions can take parameters or arguments
// if they are mapped to a concrete value, it is considered an argument
// otherwise it is considered a parameter
// Most people use argument and parameter interchangeably
fn another_function(x: i32) -> u8 {
    println!("Another function with argument: {}", x);

    // Expression scope block:
    // expressions do not include semi-colons at the end
    // if you add a semi-colon to the end of an expression it becomes a statement
    // statements do not return values
    let y = {
        let x = 3;
        x * 2
    };

    println!("Y value from expression: {}", y);

    // If expressions expect a boolean Type
    // Rust does not convert non-boolean types to boolean types like Javascript
    // eg: if y { println!("{}", y) } doesn't work, since Rust expects a boolean
    if y < 7 {
        println!("{} is less than 7", y)
    } else {
        println!("{} is greater than 7", y)
    }

    // Can use if expressions in let statements
    let x = if y < 7 { y } else { 0 };

    println!("value of conditional x is: {}", x);

    // same as "return y" in Python
    y
}

fn iterators() {
    let mut counter = 0;

    // Loop syntax that increments the above counter on every iteration
    // stores the result of the break statement in result from let statement
    let result = loop {
        counter += 1;

        // Breaks the loop when counter == 10
        if counter == 10 {
            // returns counter * 2 (should be 20)
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    // while expression syntax decrementing by 1 on each increment
    // breaks when the condition evaluates to a boolean false
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let iter_array: [i32; 6] = [10, 20, 30, 40, 50, 80];

    // expression that iterates over every element in an iterable slice
    for el in iter_array.iter() {
        println!("value is: {}", el)
    }

    // iterates over a reversed range a->(b-1)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn ownership() {
    // Make mutable variable and assign it to String
    let mut x = String::from("Hello");

    println!("X before updating: {}", x);

    // Need to pass a mutable reference to x into the function since
    // references are immutable by default in Rust
    let (a, b, c) = add_to_string(&mut x);

    println!("X after updating: {}", x);
    println!(
        "Length of x: {} \nLength of original string: {} \noriginal string: {}",
        a, b, c
    );

    // Creates a scope block
    {
        // Sets r1 to be a reference to x
        let r1 = &x;
        // since you can only have one reference to a variable at a time
        // and you can only have one mutable OR immutable reference
        // we would be unable to do the following:
        // let r2 = &x;
        // let r2 = &mut x;

        // Same applies for variable scoping, this variable is available
        // within the scope block, but will be released when the scope block terminates
        // unless it is returned somehow
        let scoped_string = String::from("Badman Tings");

        println!("Reference to r1: {}", r1);
        println!("Block scoped string: {}", scoped_string);
    } // once scope block terminates, the reference is released

    // Can reuse this variable since the ownership of 'scoped_string'
    // was released with the termination of the previous scope block
    let scoped_string = String::from("Junglist Massive");
    println!("Newly scoped string: {}", scoped_string);

    // We can now set r2 to be a reference to x again.
    let r2 = &x;
    println!("Reference to r2: {}", r2);
}

fn add_to_string(input_string: &mut String) -> (usize, usize, String) {
    // Need to use .clone() method to create a deep copy of the input variable
    // Used when you want the heap data, not just the stack data
    // let orig = input_string; makes a shallow copy and dereferences input_string
    // Otherwise explained as: input_string would be 'moved' into orig
    let orig = input_string.clone();

    // extend the input String with string literal
    input_string.push_str(", world");

    // Can return values in a tuple
    // Returns the length of the input string, orig length, and orig string
    (input_string.len(), orig.len(), orig)
}

// &str signifies a string slice type
fn slices(s: &str) -> &str {
    // Since you can't iterate over a String directly,
    // create a bytes array
    let bytes = s.as_bytes();

    // create an iterator over the bytes array, and enumerate it
    // enumerate returns a tuple (index, reference to item)
    for (i, &item) in bytes.iter().enumerate() {
        // find the first empty space and return the index
        if item == b' ' {
            // Returns a slice from 0 -> index of space
            return &s[0..i];
        }
    }

    // base case: return length of string if there are no spaces
    &s[..]
}

struct User {
    // Use String type so the instance has ownership over the value
    // If you wanted to use a &str, you would need to specify a lifetime
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // Constructs a user given an email and username parameter
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
