pub fn enum_stuff() {
    // You can have structs take enum Types
    #[derive(Debug)]
    struct IpAddress {
        kind: IpKind,
        address: String,
    }

    // creates a basic enum
    #[derive(Debug)]
    enum IpKind {
        V4,
        V6,
    }

    // Variants of an enum are namespaced by their identifier
    let four = IpKind::V4;
    let six = IpKind::V6;
    println!("ip V4 data: {:#?} \nip V6 data: {:#?}", four, six);

    let home = IpAddress {
        kind: IpKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpKind::V6,
        address: String::from("::1"),
    };

    println!(
        "Home Address: {:#?} \nLoopback address: {:#?}",
        home, loopback
    );

    // Another method to achieve the same effect
    // Advantage of this method is that you can have more flexibility over the
    // types of data that are available for each variant
    #[derive(Debug)]
    enum IpKindTerse {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let terse_home = IpKindTerse::V4(127, 0, 0, 1);
    let terse_loopback = IpKindTerse::V6(String::from("::1"));

    println!(
        "Terse Home Address: {:#?} \nTerse Loopback address: {:#?}",
        terse_home, terse_loopback
    );

    // Example of Message enum that provides more flexibility than
    // if you were to impelement the same functionality through structs
    #[derive(Debug)]
    enum Message {
        Quit,                       // No data associated with this variant
        Move { x: i32, y: i32 },    // Same as an anonymous struct with x and y pos
        Write(String),              // Same as a tuple struct
        ChangeColor(i32, i32, i32), // Also same as a tuple strut
    }

    // You can also define methods on enums
    impl Message {
        fn colorize(&self) {
            Message::ChangeColor(32, 240, 200);
        }
        fn move_it(&self) -> Message {
            let derp = Message::Quit;
            Message::Move { x: 100, y: 150 };
            derp
        }
    }

    let m = Message::Write(String::from("wuzzup"));
    m.colorize();
    m.move_it();

    // Can also use enum Option<T> to encode situations with either 'something' or nothing
    // This is super important because Rust does not have 'null' like Javascript
    // Bundled with the standard library into every prelude
    // Can use without Option::some(<t>) or Option::None syntax
    let some_number = Some(5); // type is inferred
    let some_string = Some("a string"); // type is inferred

    // can't infer type on None, so it needs to be specified in the let statement
    let absent_number: Option<i32> = None;

    println!(
        "Some number: {:?} \nSome string: {:?} \nAbsent: {:?}",
        some_number, some_string, absent_number
    );

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickle,
        Dime,
        Quarter,
    }

    // Match expression using enum variants
    // Takes a reference to Coin so I can print / match values using an array iterator
    fn get_value(c: &Coin) -> u8 {
        match c {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    
    // Create an array to hold 4 different coin options
    let test_arr: [Coin; 4] = [Coin::Penny, Coin::Nickle, Coin::Dime, Coin::Quarter];

    // Iterate over the options, get the value, and print stuff
    for c in test_arr.iter() {
        let val = get_value(c);
        println!("value of {:?} is: {} cents.", c, val);
    }

    // Matching with Option
    // good way to handle generic pattern matching
    // Needs to include a case for None otherwise will throw errors at compile time
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Matched with Option: {:?} {:?} {:?}", five, six, none );

    // the _ operator handles any case that is not included above
    // since all cases need to be handled for Rust to not throw errors
    // this is how you can handle all extra 'throwaway' cases
    let some_u8_value: u8 = 255;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("Throwaway case!"),
    }
}