// Splitting the computation in your program into multiple threads
// to run multiple tasks at the same time can improve performance,
// but it also adds complexity. Because threads can run simultaneously,
// there’s no inherent guarantee about the order in which parts of your code
// on different threads will run. This can lead to problems, such as:
//
// * Race conditions, where threads are accessing data or resources in an inconsistent order
// * Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
// * Bugs that happen only in certain situations and are hard to reproduce and fix reliably

// Programming languages implement threads in a few different ways,
// and many operating systems provide an API the language can call for  creating new threads.
// The Rust standard library uses a 1:1 model of thread implementation,
// whereby a program uses one operating system thread per one language thread.

use std::thread;
use std::time::Duration;

fn print_range(number: i32, thread_name: &str) {
    println!("hi number {} from the {} thread!", number, thread_name);
    thread::sleep(Duration::from_millis(1));
}


pub fn new_base_thread() {
    println!();
    println!("Creating a new thread using spawn");

    thread::spawn(|| {
        for i in 0..20 {
            print_range(i, "spawned")
        }
    });

    for i in 0..10 {
        print_range(i, "smain")
    }
    // Note that when the main thread of a Rust program completes,
    // all spawned threads are shut down, whether or not they have finished running.
}


pub fn thread_with_join() {
    println!();
    println!("Waiting for All Threads to Finish Using join Handles");

    let handle = std::thread::spawn(|| {
        for i in 0..10 {
            print_range(i, "spawned")
        }
    });

    // join before the next code block would block the above loop
    // until all tasks have been completed
    //
    // handle.join().unwrap();

    for i in 0..3 {
        print_range(i, "main")
    }

    //  when we call the join method on it, will wait for its thread to finish before main exits
    handle.join().unwrap();
}


pub fn thread_with_join_and_move() {
    println!();
    println!("Using move Closures with Threads");

    let v = vec![1, 2, 3, 4, 5];
    let p = v.clone();

    // By adding the move keyword before the closure,
    // we force the closure to take ownership of the values it’s using
    // rather than allowing Rust to infer that it should borrow the values
    let handle = thread::spawn(move || {
        for i in v {
            print_range(i, "spawned")
        }
    });

    // here we wait for all tasks to complete
    handle.join().unwrap();

    // By telling Rust to move ownership of v to the spawned thread,
    // we’re guaranteeing Rust that the main thread won’t use v anymore thus making the memory safe to use.
    // println!("Here's a vector: {:?}", v); // throws: value borrowed here after move

    // However, if we clone before move, we can use a copy of the original vector safely
    println!("Here's a vector: {:?}", p);
}
