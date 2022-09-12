use rand::Rng;
//use rand::*; // Brings all public functions in scope
//use rand::{Rng, ErrorKind::Transient, CryptoRng}; // nested path

mod front_of_house; // get content of module from file front_of_house
mod back_of_house;

fn server_order() {}

// up to here, only eat_at_restaurant is public.
// If we want to grant access to any of the internal functions,
// then we have to re-export the module or function to make it public.
// To do so, we used add "pub: key word to the uste statement as shown below.

// Re-export hosting module
pub use crate::front_of_house::hosting; // makes all functions in module hosting accessible from outside

pub fn eat_at_restaurant() {

    let _secret_number = rand::thread_rng().gen_range(1..=101);

    // Absolut path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // short path
    use front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();

    // bring modules into scope
    use back_of_house::{Appetizer, Breakfast};

    let mut meal = Breakfast::summer("Rye");
    println!("{:?}", meal);

    // meal.toast = String::from("What"); // fields are private by default. Compiler error
    meal.update_toast("Wheat"); // Rather, use public accessors to modify fields if needed.
    println!("{:?}", meal);

    // Enum fields are public by default
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

fn rename_import(){

    // when dealing with two imports of the same name, we need to rename at least one of them
    // to ensure that it's clear which one is used in the function.
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
       Ok(())
    }

    fn function2() -> IoResult<()> {
        Ok(())
    }

    // alternatively, we can also prefix with the imported package.
    use std::fmt;
    fn function3() -> fmt::Result {
        Ok(())
    }



}
