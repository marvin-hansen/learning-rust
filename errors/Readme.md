# Chapter 9: Error Handling

Rust groups errors into two major categories: 
* recoverable and 
* unrecoverable errors.

For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error. 

By default, use Result return type & handle errors.

Use expect & unwrap only in prototype and test code. 

In all production code, replace expect & unwrap with the appropriate error handling using ? operator, 
match over Result type or a closure for brevity.  

However, you can use unwrap in any code you know will succeed i.e. parsing a constant string for unwrapping its result type.

```rust
let home: IpAddr = "127.0.0.1".parse().unwrap();
``` 

Because the string is hardcoded, we can call unwrap safely. 

In case of a dynamic stirng, we have to handle error. 

Src:
## Error Handling in Rust
https://youtu.be/wM6o70NAWUI

## Book: 
https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html 



