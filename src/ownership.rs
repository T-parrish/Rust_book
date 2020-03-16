pub mod owner_notes {
    pub fn ownership() {
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
}
