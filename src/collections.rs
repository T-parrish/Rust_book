// Like tuples and arrays, collections can take multiple values. The difference
// is that collections point to data on the heap rather than the stack
// this means that collections do not need to have a fixed size at compile time and
// are free to grow or shrink as the program runs.
pub mod col_notes {
    // Allows you to store a variable number of values next to each other
    // Can only store items of the same type, stores them next to one another in memory
    // Similar to lists in python, but with strict type guidelines
    pub fn vectors() {
        // only need to specify the type here because we aren't passing
        // any values into the vector yet
        let mut v: Vec<i32> = Vec::new();

        // you can push onto a vector like this, but the vector needs to be mutable
        // vectors also adhere to ownership rules
        // you can't push to a vector if you hold an immutable reference to part of it
        v.push(1);
        v.push(7);
        v.push(22);

        // pop works too
        let popped = v.pop();
        println!("Popped value: {}", popped);

        // can access vector values by index
        let third: &i32 = &v[2];

        println!("The third element is: {}", third);

        // Rust is able to infer the type of data, so
        // we are able to use the vec! macro to initialize a
        // vector of i32 values with the following incantation
        let v2 = vec![1, 2, 3];

        // Can also get index values with get method and match statements for safety
        // get returns an Option<&T>
        match v2.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
        // Iterate over a vector
        for i in &v2 {
            println!("{}", i);
        }

        // update a vector while iterating
        for i in &mut v {
            println!("Before updating: {}", i);
            *i += 50;
            println!("after updating: {}", i);
        }

        // Since vectors can only take one type, enums provide a way to
        // have more flexibility with Vectors
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    // Collection of characters discussed in other modules
    // pub fn strings() {

    // }

    // Particular implementation of a map (key: value) data structure
    // pub fn hash_map() {

    // }
}
