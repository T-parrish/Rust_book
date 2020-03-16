pub mod slice_notes {
    pub fn example() {
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
    
}