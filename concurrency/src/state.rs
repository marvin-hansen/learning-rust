
// Channels in any programming language are similar to single ownership,
// because once you transfer a value down a channel,you should no longer use that value.
// Shared memory concurrency is like multiple ownership:
// multiple threads can access the same memory location at the same time.
// As you saw in Chapter 15, where smart pointers made multiple ownership possible,
// multiple ownership can add complexity because these different owners need managing.
// Rust’s type system and ownership rules greatly assist in getting this management correct.
// For an example, let’s look at mutexes, one of the more common concurrency primitives for shared memory.

// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread
// to access some data at any given time. To access the data in a mutex,
// a thread must first signal that it wants access by asking to acquire the mutex’s lock.
//
// Mutexes have a reputation for being difficult to use because you have to remember two rules:
//
// * You must attempt to acquire the lock before using the data.
// * When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
//
//Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic
// about channels. However, thanks to Rust’s type system and ownership rules, you can’t get locking and unlocking wrong.

use std::sync::{Arc,Mutex};
use std::thread;


pub fn basic_mutex(){
    println!();
    println!("Using Mutexes to Allow Access to Data from One Thread at a Time");
    let m = Mutex::new(5); // mutex guarded value

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

}

pub fn shared_mutex() {
    println!();
    println!("Atomic Reference Counting with Arc<T>");

    // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
    // The a stands for atomic, meaning it’s an atomically reference counted type.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // You might then wonder why all primitive types aren’t atomic
    // and why standard library types aren’t implemented to use Arc<T> by default.
    // The reason is that thread safety comes with a performance penalty
    // that you only want to pay when you really need to.
    // If you’re just performing operations on values within a single thread,
    // your code can run faster if it doesn’t have to enforce the guarantees atomics provide.
    for _ in 0..10 {
        // note though, we have to clone the pointer to the atomic counter
        // before moving ownership of the pointer to the thread
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move|| {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handler);
    }

    for handle in handles {
        handle.join().unwrap(); // here we basically just wait for each thread to finish
    }

    // print out the total count, incremented by each of the ten threads
    println!("counter = {}", counter.lock().unwrap());
    // counter = 10

    // You could also use this program’s structure
    // to do more complicated operations than just incrementing a counter.
    // Using this strategy, you can divide a calculation into independent parts,
    // split those parts across threads, and then use a Mutex<T> to have each thread
    // update the final result with its part.

    //counter is immutable but we could get a mutable reference to the value inside it;
    // this means Mutex<T> provides interior mutability, as the Cell family does.
    // In the same way we used RefCell<T> in Chapter 15
    // to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>.
    //
    // Another detail to note is that Rust can’t protect you from all kinds of logic errors
    // when you use Mutex<T>.
    // Recall in Chapter 15 that using Rc<T> came with the risk of creating reference cycles,
    // where two Rc<T> values refer to each other, causing memory leaks.
    // Similarly, Mutex<T> comes with the risk of creating deadlocks.
}
