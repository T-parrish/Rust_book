pub fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &t in list {
        if t > largest {
            largest = t
        }
    }

    largest
}

// Function signature with <T: PartialOrd + Copy> implies a generic T variable 
// with Trait Bounds set to accept any Type that implements PartialOrd and Copy.
// This way, the generic_largest function can accept Vectors, Arrays, Slices... as long
// as their contents implement PartialOrd and Copy
// Functions as a way to make sure that the contents are allocated on the stack
// Could also use <T: PartialOrd + Clone> to have allocations happen on the heap
// Which could potentially be expensive if you're working with lots of data
pub fn generic_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &t in list {
        if t > largest {
            largest = t
        }
    }

    largest
}

// function that returns a reference to T
// Does not allocate on the stack or heap
pub fn get_t<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    let mut target = 0;
    let mut i = 0;
    while i < list.len() {
        if list[i] < *largest {
            largest = &list[i];
            target = i;
        }
        i += 1;
    }
    &list[target]
}

#[allow(dead_code)]
pub fn generic_notes() {
    // Point is initialized with x and y both of type T
    // if they are not the same type, the compiler will throw an error
    struct Point<T> {
        x: T,
        y: T
    }

    // This is how to implement instance methods with generic type signatures
    impl<T> Point<T> {
        fn get(&self) -> &T {
            &self.x
        }
    }

    // This method only works on instances of Point with concrete type f32
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // Can also specify multiple generics on the struct
    // in this instance, x and y can be different
    struct HetPoint<T, U> {
        x: T,
        y: U
    }

    impl<T, U> HetPoint<T, U> {
        // Since T and U could be anything, if we want to have to separate
        // sets of Generic parameters we need a way to disambiguate them. You can substitute
        // new Generic variable names in the scope of the method to achieve this (ie: <V, W>)
        fn mixup<V, W>(self, other: HetPoint<V, W>) -> HetPoint<T, W> {
            HetPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

}