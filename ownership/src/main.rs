
fn main() {
    //    Ownership rules
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. here can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    stack_heap();
    copy_val_ref();
    // function_ownership();
    function_copy();
    function_move_ownership();
    function_poly_return();
    function_mut_ref();
    function_ref_with_return();
    change_read_write_ref();

    slice_function();

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

fn function_ownership() -> (){
    let s = String::from("hello");
    //takes_ownership(s); // Moves ownership to function.
    println!("{}", s); // Error:  value borrowed here after move
}

fn takes_ownership(some_string: String) ->(){
    println!("{}", some_string)
} // Here, some_string goes out of scope and `drop` is called.
// The backing memory is freed.

fn function_copy() -> (){
    // primitives implement implicitly the copy trait
    let some_integer = 5;
    makes_copy(some_integer); // an actual copy happens
    println!("{}", some_integer); // this works because some_integer never moved anywhere
} // here the integer goes out of scope and memory will be freed.

fn makes_copy(some_integer: i32){ // some_integer comes into scope
    println!("{}", some_integer)
} // Here, some_integer goes out of scope. Nothing special happens.

fn function_move_ownership() -> (){
    let s1 = gives_ownership(); // receives the ownership from function.
    let s2 = String::from("hello"); // creates new string owned by s2
    let s3 = takes_and_gives_back(s2); // moves s2 to function, but function returns ownership to s3.
    // Note, s2 is no further valid. However, s3 is because it came into scope.
    println!("{}", s1); // s1 in scope
    // println!("{}", s2); // out of scope. Would throw error:  value borrowed here after move
    println!("{}", s3); // S3 in scope and points to the string declared above.
}

fn gives_ownership() -> (String){
    let some_string = String::from("yours");
    return some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> (String){ // a_string comes into scope
   return a_string // a_string is returned and moves out to the calling function
}

fn function_poly_return() -> (){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // same as before, s1 went out of scope. S2 & len came into scope.
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); // len() returns the length of a String
    // we cannot call return (s, s.len()) as it would amount to use after move
    return (s, length)
}

fn function_ref_with_return() -> (){
    // s1 comes into scope
    let s1 = String::from("hello");

    // we only pass a read reference to function
    let len = calculate_length_ref(&s1);

    // len came into scope. Note, s1 remains in scope b/c it never moved.
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_ref(s: &String) -> usize{ // s is a read-only reference to a String
    return s.len(); // Rust de-ref s automatically.
    // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, it is not dropped.
}

fn function_mut_ref() -> (){
    // creating a new string.
    let mut s = String::from("Hello");
    // we only pass one mut ref.
    change(&mut s);
    // S still in scope b/c only a reference was passed.
    // However, the mutated string will be shown b/c the function changed the actual data
    println!("{}", s);
}

fn change(mut_string: &mut String) { // here we take a mutable reference to a string
    // note, there can only be one mut ref or many read ref at any point in time. Read & write ref cannot be mixed.
    mut_string.push_str(", world!");
}

fn change_mut_error() -> (){
    let mut s = String::from("Hello");
    let r1 = &mut s; // ok
    let r2 = &mut s; // no error yet

    // println!("{}, {}", r1, r2); // error cannot borrow `s` as mutable more than once at a time

    // As always, we can use curly brackets to create a new scope,
    // allowing for multiple mutable references, just not simultaneous ones:
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s; // ok, just one active ref.
    println!("{}", r2); // ok
}

fn change_read_write_ref() -> (){
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

   // println!("{}, {}, and {}", r1, r2, r3); //  cannot borrow `s` as mutable because it is also borrowed as immutable


    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // OK to use

    { // brackets are optional here, but used for clarity of scope.
        let r3 = &mut s; // no problem
        println!("{}", r3); // ok.
    }
}

fn function_dangle_err() -> (){
    let ref_to_nothing = dangle();
}

// This code doesn't even compile ...
//
// fn dangle() -> (&String){ // dangle returns a reference to a String
//     let s = String::from("Hello world"); // s is a new String
//     return &s // we return a reference to the String, s
// } // Here s goes out of scope and it is dropped. Its memory clears. Dangling reference ...

fn dangle() -> String {
    // solution to the dangle problem
    // 1) Return value
    // 2) Annotate lifetime
    let s = String::from("Hello world");
    return s;
}

//    Ownership rules
// 1. Each value in Rust has a variable that's called its owner.
// 2. here can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// Slices

fn slice_function(){

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    first_word_sized(&s);

    println!("{}", s);

    first_word_ref(&s);

    println!("{}", s);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_ref(&my_string_literal[0..6]);
    let word = first_word_ref(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_ref(my_string_literal);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    println!("ok!")

}

fn first_word_sized(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    let length = s.len();
    return length;
}


fn first_word_ref(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}