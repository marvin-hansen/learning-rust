use std::rc::Rc;

use crate::rc::List::{Cons, Nil};

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn test_ref_count() {
    // You have to enable multiple ownership explicitly by using the Rust type Rc<T>,
    // which is an abbreviation for reference counting.
    // The Rc<T> type keeps track of the number of references to a value to determine
    // whether or not the value is still in use.
    // If there are zero references to a value,
    // the value can be cleaned up without any references becoming invalid.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Ref count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("Ref count of a after creating b = {}", Rc::strong_count(&a));

    { // new scope
        let _c = Cons(4, Rc::clone(&a));
        println!("Ref count of a after creating c = {}", Rc::strong_count(&a));
    }

    println!("Ref count of a after c goes out of scope = {}", Rc::strong_count(&a));

    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only
}

