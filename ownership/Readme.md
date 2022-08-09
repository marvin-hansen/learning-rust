# Chapter 4 ownership 

https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
https://youtu.be/VFIOSWy93H0

Keep these rules in mind as we work through the examples that illustrate them:

## Ownership rules 
1. Each value in Rust has a variable that's called its owner.
2. here can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

