// use std::fs;
// use std::error::Error;
use std::process;
use std::env;

use d012::Config;

// Lib crate isn't recognized:
// https://github.com/rust-lang/vscode-rust/issues/686
//

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() != 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Ok(Config { query, filename })
//     }
// }



fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        println!("参数解析错误了，所以这里将抛错退出");

        // 将错误打印到标准错误
        eprintln!("Problem parsing arguments: {}", err);
        
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = d012::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}


// fn run(config: Config) {
//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.filename)?;
//     println!("With text:\n{}", contents);
//     Ok(())
// }
