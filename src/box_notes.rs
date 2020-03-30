
// box (Type: Box<T>) is the most straightforward 'smart pointer', where the pointer
// remains on the stack and the data it refers to is stored on the heap
// main use cases include: Transferring large quantities of data without copying
// When you want to own a value and you only care that it implements a specific trait
// When you have a Type whose size can't be known at compile time, but
// want to use it in a context that requires an exact size.
#[allow(dead_code)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

// When the box goes out of scope, the pointer and the data are deallocated.
#[allow(dead_code)]
pub fn new_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn recursive_type() {
    // Cons, short for construct function comes from Lisp
    // constructs a new pair from its two arguments
    // two arguments are usually a single argument and another pair
    // 'to cons x onto y'

    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
}


// Creating a custom smart pointer
use std::ops::Deref;

// Tuple struct of type T
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    #[allow(dead_code)]
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Without the Deref trait, the compiler can only dereference '&' references.
impl<T> Deref for MyBox<T> {
    // Associated type for the Deref Trait
    type Target = T;

    // Borrows self and returns a reference to the inner data, which 
    // the compiler is then able to properly dereference with *
    // This allows the Type to retain ownership of its data
    fn deref(&self) -> &T {
        &self.0
    }
}

// Example for when we want a custom drop implementation of the Drop Trait
#[derive(Debug)]
pub struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[allow(dead_code)]
pub fn hello(name: &str) {
    format!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_deref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        // cant compare values to references
        // assert_eq!(5, y) will throw a compiler error
        assert_eq!(5, *y);
    }

    #[test]
    fn box_deref() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn custom_box_deref() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_coercion() {
        // When the Deref Trait is defined for all Types involved, Rust will analyze
        // the types and use Deref::deref as many times as necessary to get a 
        // reference to match the function parameter's Type.
        let m = MyBox::new(String::from("Rust"));
        // Since we implemented Deref on MyBox, Rust can turn &MyBox<String>
        // into &String, and the std library implements Deref on String
        // so the &String is coerced into &str, which is an acceptable Type
        // in hello's function signature.
        let x = hello(&m);
        // Without Deref coercion, this is how you would have to handle
        // MyBox<String> -> &str transformations.
        let y = hello(&(*m)[..]);

        assert_eq!(x, y);
    }

    #[test]
    #[ignore]
    fn drop_early() {
        // New instance of CustomSmartPointer
        let c = CustomSmartPointer { data: String::from("other stuff") };

        println!("Data from custom smart pointer: {:?}", c);
        // Can't call drop method directly on an instance of c (eg c.drop())
        // have to call it like this:
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
