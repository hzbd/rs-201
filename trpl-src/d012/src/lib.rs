use std::fs;
use std::error::Error;


pub struct Config {
    query: String,
    filename: String,
    // pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        //
        // // export CASE_INSENSITIVE=True
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        // println!("{}", line);
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);

    for line in search(&config.query, &contents) {
        println!("search result => {}", line);
    }

    Ok(())
}

pub fn add_two(num:i32) -> i32 {
    num
}
