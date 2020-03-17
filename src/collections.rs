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
        println!("Popped value: {:?}", popped);

        v.push(33);

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
        #[derive(Debug)]
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

        println!("First cell value: {:?}", row.get(0));
    }

    // Collection of characters discussed in other modules, provided by Rust’s standard library
    // Both String and &str are growable, mutable, owned, UTF-8 encoded string types
    pub fn strings() {
        // Different ways to create a String
        let data = "What's goin on";
        let s = data.to_string();
        let s2 = "help I'm a String".to_string();

        println!("String stuff: \n{} \n{}", s, s2);

        let mut s3 = String::from("Append to me");
        // You can push string literals onto other Strings
        s3.push_str(", I double dog dare you");

        // You can also push single characters onto a String with push method
        // Important to note the single quotes as opposed to double quotes
        s3.push('.');

        // Concatenating strings in this fashion results in the scope losing ownership of s2
        let s4 = s2 + " " + &s3;
        println!("Phrase after concatenating: \n{}", s4);

        // better way to format strings
        let formatted = format!("{}\n{}\n{}", s, s3, s4);
        println!("{}", formatted);

        // You can iterate over String chars or bytes out of the box.
        // Grapheme clusters can be handled with external crates and deps
        // This results in a bunch of extra upfront complexity, but
        // prevents downstream errors from non-ascii characters
        let hello = "Здравствуйте";
        for c in hello.chars() {
            println!("char: {}", c)
        }

        for b in hello.bytes() {
            println!("byte: {}", b)
        }
    }

    use std::collections::HashMap;

    // Particular implementation of a map (key: value) data structure
    // all k must have the same type, all v must have the same type
    pub fn hash_map() {
        let mut scores = HashMap::new();
        // Insert k: v pairs into hashmap with insert method
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // Since all keys need to be unique, this updates the k:v pair
        scores.insert(String::from("Blue"), 12);

        // Checks if Yellow is in the keys, add yellow: 50 if it isn't
        // returns a mutable reference to the new value when triggered
        scores.entry(String::from("Yellow")).or_insert(50);

        // Another way to construct hash maps with iterators and zip
        let teams = vec![String::from("Chelsea"), String::from("Manchester United")];
        let scores = vec![32, 42];

        // Need to use HashMap<_, _> type def because collect can collect into
        // many different data structures. Rust will infer the types from the vectors
        // For types that implement the Copy trait (eg i32), the values with be copied
        // for types that don't implement Copy, the values are moved into the hashmap
        let mapped: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

        // Get returns an Option<&V>
        let t = match mapped.get(&String::from("Chelsea")) {
            None => None,
            val => val,
        };

        println!("Chelsea scored {:?} points.", t);

        for (k, v) in &mapped {
            println!("Team: {} - Score: {}", k, v);
        }

        println!("{:?}", mapped);

        // Method to count word occurrences in text
        let text = "hello world wonderful world";
        let mut word_counter = HashMap::new();

        for word in text.split_whitespace() {
            let count = word_counter.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", word_counter);

    }
}
