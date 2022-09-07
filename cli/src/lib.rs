use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) ->  Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // return results

    // functional equivalent of the above code block
    return contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Config {
    query: String,
    file_path: String,
    case_sensitive: bool,
}

impl Config {
    // new is a simple constructor only checking if enough parameters have been passed
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments. use: -- word file.txt ");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // read from env. variable or set to false

        Ok(Config { query, file_path, case_sensitive })
    }

    // build is a more comprehensive constructor that checks each argument individually
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        args.next();

        // iterating through each argument allows specific error messages
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let case_sensitive = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
}

impl Config {
    pub fn query(&self) -> String {
        return self.query.clone()
    }

    pub fn file_path(&self) -> String {
        return self.file_path.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents)
        );
    }
}