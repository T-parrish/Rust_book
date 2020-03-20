use std::io;
use std::io::Read;
use std::fs::File;
use std::net::IpAddr;
use std::fs;
use std::io::ErrorKind;

pub fn read_username_from_file() -> Result<String, io::Error> {
    // attempts to read hello.txt which returns a Result<T, E>
    // the ? operator breaks and propagates the error without match expressions
    // the ? also converts the error into the error type specified in the return Type
    // in this case, io::Error
    // Cant use ? in main unless fn main() -> Result<(), Box<dyn Error>> {}
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // since read to string also returns a Result<T, E>
    // use the ? operator to propagate the error if it occurs
    f.read_to_string(&mut s)?;
    println!("With ? operator: {}", s);

    // calling unwrap on a function that returns a Result will either
    // evaluate to the successful value or panic in case of error
    // Typically used as a placeholder until you want to better define error handling
    // or in cases where there is no way you can trigger an error
    let mut s2 = String::new();
    let f2 = File::open("hello.txt").unwrap().read_to_string(&mut s2);
    
    println!("Successfully unwrapped: {:?}", f2);

    // can chain the methods together with ? operator 
    let mut s3 = String::new();
    File::open("hello.txt")?.read_to_string(&mut s3)?;

    // easier way to read a file to String 
    // doesn't involve opening and reading, reads bytes directly to string.
    let s4: String = fs::read_to_string("hello.txt")?.parse().unwrap();
    println!("String derived from fs::read_to_string: {}", s4);


    // Way to specify and handle different error types
    let f5 = File::open("hello.txt");
    let f5 = match f5 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("String from match statement with Error Types: {:?}", f5);

    // Verbose way to match every step of the process when reading file data
    let f6 = File::open("hello.txt");
    let mut f6 = match f6 {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s6 = String::new();
    match f6.read_to_string(&mut s6) {
        Ok(_) => Ok(s6),
        Err(e) => return Err(e)
    }
}

pub fn parsing_notes() {
    // Case where it's acceptable to use unwrap: the compiler can't
    // know that "127.0.0.1" will always be a valid ip address. Since it
    // will always be valid, use unwrap to get the value tersely
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home: {:?}",home);
}

// Pattern to validate data
pub fn struct_handling() {
    #[derive(Debug)]
    pub struct Guess {
        value: u16
    }

    impl Guess {
        // Associated (read: static in Python) function that validates the input
        // to be between 1 and 100, then returns an instance of Guess
        // if the value falls outside that range, panic
        pub fn new(value: u16) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            // new() evaluates to an instance of guess with the validated value
            Guess {
                value
            }
        }

        // Instance method that returns its value
        pub fn value(&self) -> u16 {
            self.value
        }
    }

    let testing = Guess { value: 22 };
    println!("Testing initialization of Guess without ::new(): {:?}", testing);

    let t2 = Guess::new(28);
    println!("Testing initialization of Guess with ::new(): {:?}", t2);

    let t3 = t2.value();
    println!("Value of Guess: {}", t3);

}