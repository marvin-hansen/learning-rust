
fn main() {
    //    Ownership rules
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. here can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    stack_heap();

    copy_val_ref();

}

fn stack_heap() -> (){
    { // s not declared yet
        // s string literal, fixed in size, and stored directly in the binary.
        // stored on the stack.
        let s: &str = "hello"; // s is valid from this point forward

        // If you need a dynamically sized string, you use the string type and call new;
        // String is stored on the heap.
        let s: String = String::from("hello");
        // s is allocated and de-allocated automatically.

        // do stuff with s
    } // this scope is now over, and s is no longer valid
}

fn copy_val_ref() -> (){
    let x = 5;
    println!("create X: {}", x);

    let y = x; // implicit copy b/c primitive data on the stack.
    println!("copy X");

    // creates a dynamically sized string on the heap
    let s1 = String::from("hello");
    println!("create s1: {}", s1);

    // moves string to new owner s2 and invalidates s1. (Rule #2: One owner at a time)
    let s2 = s1;
    println!("Move s1");

    // println!("print s1: {}", s1); // error: value borrowed here after move

    println!("print s2: {}", s2);

    // what if we want an actual duplicate?

    // clone the entire string and assign its to a new owner, s3
    let s3 = s2.clone();
    println!("print s3: {}", s3);
}

fn takes_ownership(some_string: string) ->(){
    println!("{}", some_string)
}