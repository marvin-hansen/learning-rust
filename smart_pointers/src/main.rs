use std::ops::Deref;
use mem_leak::test_mut_ref_list;
use crate::lib::CustomSmartPointer;
use crate::List::{Cons, Nil};
use crate::mut_own::test_mut_owners;
use crate::rc::test_ref_count;
use crate::tree::test_weak_ref_tree;

mod lib;
mod rc;
mod msg;
mod mut_own;
mod mem_leak;
mod tree;

fn a_box() {
    // Boxes don’t have performance overhead, other than storing their data on the heap.
    //  You’ll use them most often in these situations:
    //
    // * When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // * When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // * When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    // We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap
    let b = Box::new(5);
    println!("Box value: {}", b);
}


enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons_list() {

    // Doesn't compile because of infinite recursion
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // the problem is that the compiler cannot determine the memory size of a recursive constructor call

    // A box, however, has a fixed size on the stack and points to data on the heap
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn deref() {
    let x = 5;
    let y = Box::new(x); // we have to use box b/c y = &x fails because we cannot compare a number to a reference. Obviously.

    assert_eq!(5, x);
    assert_eq!(5, *y); // However, we can deref a box and get the pointed value from it.
    // Note though, box points at a *copied* value of x so there is no actual memory gain.
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // we have to overwrite deref with a type target to make deref work for MyBox
    type Target = T;
    // Defines an associated type for the Deref trait
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn deref_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // compilers rewrites this to *(y.deref())
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // here string is automatically coercive from &MyBox<String> into a &String reference
    // note this only works when the Deref trait is implemented. Without deref, you would write &(*m)

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    //
    // * From &T to &U when T: Deref<Target=U>
    // * From &mut T to &mut U when T: DerefMut<Target=U>
    // * From &mut T to &U when T: Deref<Target=U>

    // The first two cases are the same as each other except that the second implements mutability.
    // The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently.
    // The second case states that the same deref coercion happens for mutable references.
    //
    // The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
    // But the reverse is not possible: immutable references will never coerce to mutable references.
    // Because of the borrowing rules, if you have a mutable reference,
    // that mutable reference must be the only reference to that data
}

fn test_custom_smart_pointer() {
    let _c = CustomSmartPointer::new(String::from("my stuff"));

    {
        let d = CustomSmartPointer::new(String::from("other stuff"));
        println!("CustomSmartPointers created.");

        std::mem::drop(d) // drop cannot be called directly, all you can do is call mem::drop to prevent double free
    } // d goes out of scope

    println!("CustomSmartPointers created.");
} // c gets out of scope and calls drop() automatically


fn main() {
    println!("Hello, smart pointers!");
    a_box();
    cons_list();
    deref();
    deref_my_box();
    deref_coercion();
    test_custom_smart_pointer();
    test_ref_count();
    test_mut_owners();
    test_mut_ref_list();
    test_weak_ref_tree();
}
