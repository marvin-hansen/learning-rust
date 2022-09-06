use std::fmt::Display;

fn main() {
    println!("Hello, Lifeline!");
    proper_ref();
    test_longest();
    test_longest_mixed_lifetime();
    test_struct_lifetime();
    test_static_lifetime();
}

fn _dangling_ref(){
    let r: &i32;

    {
        let x = 5;
        r = &x;
    } // x gets out of scope

   //  println!("x does not live long enough x is {}", *r); // r is a dangling reference as it would go to null
}

fn proper_ref(){
    let x = 5;             // ----------+-- 'b
                                //           |
    let r = &x;           // --+-- 'a  |
                                //   |       |
    println!("r: {}", r);       //   |       |
                                // --+       |
}                               // ----------+

fn test_longest(){
    let string1 = String::from("hello");
    let string2 = String::from("world!");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest : {}", result);
}

fn test_longest_mixed_lifetime(){
    let string1 = String::from("hello");

    {
        let string2 = String::from("world!");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest : {}", result);
    } // string2 lifetime ends here

}// string1 lifetime ends here


fn _test_longest_broken_lifetime(){
    let string1 = String::from("hello");
    let mut result: &str;

    {
        let string2 = String::from("world!");
        result = longest(string1.as_str(), string2.as_str());
    } // string2 lifetime ends here

   // println!("The longest : {}", result); // error:  borrowed value does not live long enough

}// string1 & result lifetime ends here

// Lifetime Annotation Syntax
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ImportantExcerpt<'a>{
    part: &'a str,
}


impl<'a>ImportantExcerpt<'a> { // don't need lifetime annotation b/c compiler can infer from &self
    fn return_part(&self, announcement: &str) -> & str{
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn test_struct_lifetime(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find first sentence.");

    let excerpt = ImportantExcerpt {part: first_sentence };
    let part = excerpt.return_part("Wheat");

    println!("First sentence: {}", part);

}


// no lifetime annotation needed b/c compiler can deterministically generate a lifetime annotation
fn first_word(s: &str) -> &str {
// fn first_word<'a>(s: &'a str) -> &'a str { // generated annotations
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Lifetime elision rules
// 1. Each parameter that is a reference gets its own lifetime parameter annotation
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
// the lifetime of self is assigned to all output lifetime parameters.


fn test_static_lifetime(){
    // static lifetime lasts as long as the program runs
    let s: &'static str = "I have a static lifetime";
    println!("{}", s);
}


// Generics & trait bound & lifetime parameters annotation
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
    where T: Display {

    println!("Announcement: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
