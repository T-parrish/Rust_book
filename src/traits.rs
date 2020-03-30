pub fn trait_notes() {
    // Definition of Summary Trait that expresses summarize behavior
    // Similar to normal function defs except you end with a semi-colon
    // Traits are similar to 'interfaces' in other languages
    // Seems like ABC (abstract base classes) in Python
    pub trait Summary {
        // Placeholder, means that every Type with this Trait will
        // need to implement behavior if a default is not specified (see below)
        fn summarize(&self) -> String;
        // The Summary trait will express sum_default behavior on every
        // Type with the Summary Trait, but you can still override
        // the behavior on each individual Type
        fn sum_default(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // syntax to implement a Trait on a Type
    // impl <Trait Name> for <Type>
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };
    // Will express the summarize behavior implemented on NewsArticle
    println!("New article available! {}", article.summarize());

    // Will express the default behavior implemented on Summary Trait
    println!("New article available! {}", article.sum_default());

    // Major benefit of traits is that you can have generics
    // be Types with a specific Trait expression
    // For example, this function accepts anything that implements the Summary Trait
    fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Same as above, but using a Generic with a Trait Bound
    #[allow(dead_code)]
    fn trait_bound<T: Summary>(item: T) {
        println!(
            "Breaking news, but using a Trait Bound! {}",
            item.summarize()
        );
    }

    // NewsArticle implements Summary, so it can be passed into notify
    notify(article);

    // You can also have functions return items that implement a Trait
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    println!(
        "Returned an item that implements Summary Trait: {:#?}",
        returns_summarizable().summarize()
    );
}
