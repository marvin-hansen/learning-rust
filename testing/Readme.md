# Chapter 11: Testing 

In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that
“Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.” 
That doesn’t mean we shouldn’t try to test as much as we can!

Correctness in our programs is the extent to which our code does what we intend it to do. 
Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove. 
Rust’s type system shoulders a huge part of this burden, but the type system cannot catch everything. 
As such, Rust includes support for writing automated software tests.

Tests are Rust functions that verify that the non-test code is functioning in the expected manner. 
The bodies of test functions typically perform these three actions:

* Set up any needed data or state.
* Run the code you want to test.
* Assert the results are what you expect.

* Let’s look at the features Rust provides specifically for writing tests that take these actions, 
* which include the test attribute, a few macros, and the should_panic attribute.

Src:
## Testing in Rust - Part I
https://youtu.be/18-7NoNPO30 

## Testing in Rust - Part II
https://youtu.be/-L4nKAlmH3M

## Book:
https://doc.rust-lang.org/stable/book/ch11-00-testing.html
