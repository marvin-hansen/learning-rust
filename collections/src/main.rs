use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    test_create_vec();
    test_unsafe_vec_access();
    test_safe_vec_access();
    test_iterate_vec();
    test_mixed_type_vec();
    test_string();
    test_string_segmentation();
    test_hashmap();
    test_update_hashmap();
    test_string_hashmap();
}


fn test_create_vec() {
    let _a = [1, 2, 3]; // standard array

    let _v: Vec<i32> = Vec::new(); // vector type

    {
        let _v2 = vec![1, 2, 3]; // very similar to array; type can be infered
    } // v2 will be dropped here

    println!();
}


fn test_unsafe_vec_access() {
    let mut v: Vec<i32> = Vec::new(); // type
    v.push(1);
    v.push(2);
    v.push(2);

    // access elements directly via index
    let third = &v[2];
    // v.push(6); // error:  mutable borrow occurs here b/c there already is a immutable borrowed reference
    println!("The third element: {}", third);

    // let invalid = &v[30]; // throws runtime out of bound error
    println!();
}


fn test_safe_vec_access() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("Handling some type");
    match v.get(2) { // returns Result that must be handled gracefully
        Some(third) => println!("The third element: {}", third),
        None => print!("There is no third element"),
    }

    println!("Handling none type");
    match v.get(20) { // returns Result that must be handled gracefully
        Some(third) => println!("The third element: {}", third),
        None => println!("There is no element"),
    }
    println!();
}


fn test_iterate_vec() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("Immutable iterating...");
    for i in &v {
        println!("{}", i);
    }
    println!();

    println!("Mutable iterating...");
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
    println!();
}

// we can store values in an enum
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn test_mixed_type_vec() {
    println!("Mixed Type Vector...");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];

    test_cell(&row[1]);

    test_all_cell(&row[0]);
    test_all_cell(&row[1]);
    test_all_cell(&row[2]);
}


fn test_cell(cell: &SpreadsheetCell) {
    match cell {
        SpreadsheetCell::Int(i) => println!("Integer: {}", i),
        _ => println!("Not an integer"),
    }
}


fn test_all_cell(cell: &SpreadsheetCell) {
    match cell {
        SpreadsheetCell::Int(n) => println!("Integer: {}", n),
        SpreadsheetCell::Float(n) => println!("Float: {}", n),
        SpreadsheetCell::Text(n) => println!("String: {}", n)
    }
}


fn test_string() {
    // In Rust, String are stored as a collection of UTF-8 encoded bytes

    let _s1 = String::new(); // new empty string
    let s2: &str = "Initial content"; // slice of string
    let _s3 = s2.to_string(); // convert slice into string
    let _s4 = String::from("hello");

    // just like a vector, a string can grow and shrink
    let mut s: String = String::from("foo");
    s.push_str("bar");
    s.push('!');// append a char

    println!("Resulting string: {}", s);

    // string concatenation
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    println!("First string: {}", s1);
    println!("Second string: {}", s2);

    // we have to concatenate after println
    // to prevent a value borrowed after move error.
    let s3 = s1 + &s2; // here we move ownership of s1 to s2,
    // and then append everything from s2 to s3.
    println!("Resulting string: {}", s3);

    // Alternatively, the format macro prevents move
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = format!("{}{}", s1, s2); // no move, no problem

    println!("First string: {}", s1);
    println!("Second string: {}", s2);
    println!("Resulting string: {}", s3); // safe
}


fn test_string_segmentation() {
    let hello = String::from("नमस्ते");
    // Bytes
    // 224 164 168 224 164 174 224 164 184 224 165 141 224 164 164 224 165 135

    println!("Print all bytes ");
    for b in hello.as_bytes() {
        println!("{}", b);
    }

    // Scalar values
    // न म स ् त े
    println!("Print all scalars ");
    for c in hello.chars() {
        println!("{}", c);
    }

    // Grapheme clusters
    // न म स् ते
    println!("Print all Grapheme clusters ");
    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}


fn test_hashmap() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10); // moves string into hashmap
    scores.insert(yellow, 50);

    // println!("{}", blue) // error: value borrowed here after move

    println!("Print HashMap");
    for (key, value) in scores.iter() {
        println!("Team: {}, Score: {}", key, value);
    }
}


fn test_update_hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    println!("insert first entry");
    scores.insert(String::from("blue"), 10); // moves string into hashmap

    println!("update first entry");
    scores.insert(String::from("blue"), 20);
    for (key, value) in scores.iter() { println!("Team: {}, Score: {}", key, value); }

    println!("insert second entry if it doesn't exist");
    scores.entry(String::from("yellow")).or_insert(30);
    for (key, value) in scores.iter() { println!("Team: {}, Score: {}", key, value); }

    // no overwrite of stored values in map
    println!("get or insert  first entry");
    scores.entry(String::from("yellow")).or_insert(40);
    for (key, value) in scores.iter() { println!("Team: {}, Score: {}", key, value); }
}


fn test_string_hashmap() {
    let text = "Hello world wonderful world Hello again";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}
