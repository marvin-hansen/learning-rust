
extern crate core;

use std::fs::{self, File};
use std::io;
use std::io::Read;

const UNRECOVERABLE: bool = false;

fn main() {
    if UNRECOVERABLE {
        unrecoverable();
    }

    let path = "hello.txt";

    recoverable(path);
    simple_panic(path);
    read_username_from_file(path).expect("Expected Username string");
    read_username_from_file_simplified(path).expect("Expected username string");
    read_username_from_file_simplest(path).expect("Expected username string");
}

fn unrecoverable() {
    panic!("Crash and burn!");
}

fn simple_panic(path: &str) {

    let f = File::open(path);

    // handle the result type
    let _f = match f {
        Ok(f) => f,
        Err(e) => panic!("Failed to open {}: {}", path, e),
    };

    // shorthand
    let _fx = File::open(path).unwrap();
    println!("File unwrapped");

}

fn recoverable(path: &str) {
    let f = File::open(path);

    let _f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => match File::create(path) {
                Ok(file) => file,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // closure version of the above function
    let mut f = File::open(path).unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            fs::File::create(path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(s) => {
            println!("File content {}", s);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }


}

// Type alias for return type
type StringResult = Result<String, io::Error> ;

fn read_username_from_file(path: &str) -> StringResult {
    let f = File::open(path);

    let mut f = match f{
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }};

    let mut s = String::new();

    return match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };

}

fn read_username_from_file_simplified(path: &str) -> StringResult {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    return  Ok(s);
}

fn read_username_from_file_simplest(path: &str) -> StringResult {
    fs::read_to_string(path) // convenient string reader method in std::fs package
}



