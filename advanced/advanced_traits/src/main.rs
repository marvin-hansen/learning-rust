mod lib;

use std::borrow::Borrow;
use std::ops::Add;

//
// Specifying Placeholder Types in Trait Definitions with Associated Types

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // overwrite associated type
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn test_trait() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

//
//  Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("This is your wizard. Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn test_fqn_trait() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn test_animal_trait() {
    println!("A baby dog is called a {}", Dog::baby_name());

    // In general, fully qualified syntax is defined as follows:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

//
// Default Generic Type Parameters and Operator Overloading

use std::fmt;
use std::fmt::{Display, Formatter};

trait OutlinePrint: fmt::Display {
    // default implementation for all types implementing Display
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

//
// Using Supertraits to Require One Trait’s Functionality Within Another Trait
//
//Sometimes, you might write a trait definition that depends on another trait:
// for a type to implement the first trait,
// you want to require that type to also implement the second trait.
// You would do this so that your trait definition can make use of the associated items
// of the second trait. The trait your trait definition
// is relying on is called a supertrait of your trait.

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {} // dispatches to default implementation

fn test_outline_point() {
    let point = Point { x: 1, y: 2 };
    point.outline_print();
}

// Using the Newtype Pattern to Implement External Traits on External Types
//

// we mentioned the orphan rule that states we’re only allowed to implement a trait on a type
// if either the trait or the type are local to our crate.
// It’s possible to get around this restriction using the newtype pattern,
// which involves creating a new type in a tuple struct.
// The tuple struct will have one field and be a thin wrapper around the type
// we want to implement a trait for.
// Then the wrapper type is local to our crate,
// and we can implement the trait on the wrapper.

// As an example, let’s say we want to implement Display on Vec<T>,
// which the orphan rule prevents us from doing directly because the Display trait and the Vec<T>
// type are defined outside our crate.
// We can make a Wrapper struct that holds an instance of Vec<T>;
// then we can implement Display on Wrapper and use the Vec<T> value

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    // The implementation of Display uses self.0 to access the inner Vec<T>,
    // because Wrapper is a tuple struct and Vec<T> is the item at index 0 in the tuple.
    // Then we can use the functionality of the Display type on Wrapper
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

fn test_vec_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("vector wrapper = {}", w);
}

// Creating Type Synonyms with Type Aliases
//
type Kilometers = i32;

fn test_type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn test_long_trait_alias() {
    let f: Thunk = Box::new(|| println!("hi"));
    takes_long_type(f);
}

fn takes_long_type(f: Thunk) {
    // --snip--
}


// Function Pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

// We’ve talked about how to pass closures to functions;
// you can also pass regular functions to functions!
// This technique is useful when you want to pass a function you’ve already defined rather
// than defining a new closure. Functions coerce to the type fn (with a lowercase f),
// not to be confused with the Fn closure trait.
// The fn type is called a function pointer.
// Passing functions with function pointers will allow you to use functions as arguments to other functions.

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_function_pointer(){
    // Unlike closures, fn is a type rather than a trait,
    // so we specify fn as the parameter type directly rather than declaring a generic type parameter
    // with one of the Fn traits as a trait bound.
    let answer = do_twice(add_one, 5);

    println!("The answer is: {:?}", answer);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Status {
    Value(u32),
    Stop,
}

fn test_enum_init() {
    let list_of_statuses: Vec<Status> = (0u32..10).map(Status::Value).collect();

    for status in list_of_statuses {
        println!("{:?}", status);
    }

}

// Returning Closures
//
// Closures are represented by traits, which means you can’t return closures directly.
// In most cases where you might want to return a trait,
// you can instead use the concrete type that implements the trait as the return value of the function.
// However, you can’t do that with closures because they don’t have a concrete type that is returnable;
// you’re not allowed to use the function pointer fn as a return type, for example.

// Instead, We can use a trait object:
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn test_return_closure() {
    let f: Box<dyn Fn(i32) -> i32> = return_closure();
    println!("The answer is: {:?}", f(5));
}


fn main() {
    test_trait();
    test_fqn_trait();
    test_animal_trait();
    test_outline_point();
    test_vec_wrapper();
    test_type_alias();
    test_long_trait_alias();
    test_function_pointer();
    test_enum_init();
    test_return_closure();
}
