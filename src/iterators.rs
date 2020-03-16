pub fn iterator() {
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