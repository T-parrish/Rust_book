pub fn struct_exploration() {
    struct User {
        // Use String type so the instance has ownership over the value
        // If you wanted to use a &str, you would need to specify a lifetime
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    // Function that returns a User struct instance
    fn build_user(email: String, username: String) -> User {
        // Constructs a user given an email and username parameter
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

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