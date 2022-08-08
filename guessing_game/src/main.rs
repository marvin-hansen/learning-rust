use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // generate a secret number
    let secret_number = rand::thread_rng().gen_range(1..=101);
    // println!("The secret number is: {}!", secret_number);

    loop {
        println!("Please input your guess as a positive number!");

        // mutable state to store the guess
        let mut read_guess: String = String::new();

        // read the guess from terminal
        io::stdin()
            .read_line(&mut read_guess)
            .expect("Failed to read line");

        // convert string to unsigned integer
        let guess:u32 = match read_guess.trim().parse(){
            Ok(num) => num, // if ok, return the number
            Err(_) => continue, // if not, skip & read line again in next inter of the loop
        };

        // compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => {println!("{}", "Too small!".red())}
            Ordering::Greater => { print!("{}","Too big!".red());}
            Ordering::Equal => {
                println!("{}","You win!".green());
                break; // exit the program
            }
        }
    }
}
