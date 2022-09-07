mod lib;

use std::env;
use std::process;
use crate::lib::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    // here we just pass the arguments to create a new config
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // redirects output to stdout
        process::exit(1);
    });

    // here we capture environment variables and pass them to the config build method
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprintln!("Application error: {e}"); // redirects output to stdout i.e. terminal
        process::exit(1);
    }

}
