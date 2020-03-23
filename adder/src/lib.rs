// Run tests using cargo test
// for more granular control over number of threads,
// cargo test -- --test-threads=1

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    // pull Rectangle into scope
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // Evaluates truthiness of expression
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // Evaluates truthiness of expression
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn should_add_two() {
        // Assert equals -> 4 should be == add_two(2)
        assert_eq!(4, add_two(2));
        // Assert not equals -> 5 should != add_two(2)
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // In case of failure, can specify a message to send
        // Useful for debugging
        assert!(
            result.contains("Carol"),
            "Greeting did not contain the name Carol, recieved {} instead",
            result
        );
    }

    // This is a test for proper error handling
    // in this case, we check to see that the Guess::new() function
    // should cause a panic because the input is outside the 1 - 100 range
    // 'expected' will check to to see that the panic messages are equal
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Can also ignore tests that might be expensive or take a long time to run
    // These tests will have to be run intentionally using cargo test -- --ignored
    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
