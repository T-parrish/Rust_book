fn fix_incorrect_order() {
    cook_order();
    super::serve_order(); // super reverts to parent of current module
}

fn cook_order() {}

// default for enum variants is to be public since they wouldn't
// be very useful otherwise. Struct fields, however, default to private
#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}