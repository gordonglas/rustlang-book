// Demonstrate "paths"...

// Note that you only need to load a file using a mod declaration once in your module tree,
// unlike #include in C/C++ (which must be present in each "translation unit".)
mod front_of_house;

// Demonstrate "use" to avoid typing a full path...
// Can also "re-export" by prefixing with "pub ".
// With "pub use", we can write our code with one structure but expose a different structure.
// Doing so makes our library well organized for programmers working on the library and programmers calling the library.
// See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use
// See: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
pub use crate::front_of_house::hosting;
// ...now we could call it within "eat_at_restaurant()" like...
//   hosting::add_to_waitlist();

// Demonstrate "use as" to create an alias to a path...
// Good for avoiding name clashes.
use std::io::Result as IoResult;

// Demonstrate bringing multiple paths into scope on the same line...
// Instead of...
//use std::cmp::Ordering;
//use std::io;
// ...you can do this...
//use std::{cmp::Ordering, io};

// Demonstrate glob operator to bring all public items into scope...
// But glob can make it harder to tell what names are in scope and
// where a name used in your program was defined.
// The glob operator is often used when testing to bring everything
// under test into the tests module.
// See: https://doc.rust-lang.org/book/ch11-01-writing-tests.html#how-to-write-tests
//use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Demonstrate public/private with struct...
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Demonstrate public enum...
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Demonstrate "super::" (equivalent to file system's "..") ...
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // Demonstrate public/private with struct...
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

    // Demonstrate public enum...
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
