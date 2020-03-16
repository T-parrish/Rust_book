// Functions can take parameters or arguments
// if they are mapped to a concrete value, it is considered an argument
// otherwise it is considered a parameter
// Most people use argument and parameter interchangeably
pub fn another_function(x: i32) -> u8 {
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