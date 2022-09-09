mod lib;
mod poly;

use lib::*;
use poly::*;

fn main() {

     let screen = Screen{
         components: vec![
             Box::new(SelectBox{
                 width: 75,
                 height: 10,
                 options: vec![
                     String::from("Yes"),
                     String::from("Maybe"),
                     String::from("No"),
                 ]
             }),
             Box::new(Button{
                 width: 50,
                 height: 10,
                 label: String::from("OK"),
             }),
         ],
     };

    // run the screen with all components
    screen.run()

    // When we wrote the library, we didn’t know that someone might add the SelectBox type,
    // but our Screen implementation was able to operate on the new type and draw it
    // because SelectBox implements the Draw trait, which means it implements the draw method.
    // Note, though, Rust won’t compile our code if the values don’t implement the traits that the trait objects need.
}


// Recall in the “Performance of Code Using Generics” section in Chapter 10 our discussion on the
// monomorphization process performed by the compiler when we use trait bounds on generics:
// the compiler generates nongeneric implementations of functions and methods for each
// concrete type that we use in place of a generic type parameter.
// The code that results from monomorphization is doing static dispatch,
// which is when the compiler knows what method you’re calling at compile time.
//
// This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time
// which method you’re calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
// When we use trait objects, Rust must use dynamic dispatch.
// The compiler doesn’t know all the types that might be used with the code that’s using trait objects,
// so it doesn’t know which method implemented on which type to call.
// Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call.
// This lookup incurs a runtime cost that doesn’t occur with static dispatch.
// Dynamic dispatch also prevents the compiler from choosing to inline a method’s code,
// which in turn prevents some optimizations.
// However, we did get extra flexibility in the code