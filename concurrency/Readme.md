# Chapter 16: Concurrency

By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors. 
Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs,
incorrect code will refuse to compile and present an error explaining the problem. 
As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production. 
We’ve nicknamed this aspect of Rust fearless concurrency. 
Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

Here are the topics we’ll cover in this chapter:

* How to create threads to run multiple pieces of code at the same time
* Message-passing concurrency, where channels send messages between threads
* Shared-state concurrency, where multiple threads have access to some piece of data
* The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

## Concurrency in Rust - Creating Threads
https://youtu.be/06WcsNPUNC8

## Book 
https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html#fearless-concurrency