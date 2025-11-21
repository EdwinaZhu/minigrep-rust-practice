use std::error::Error;
use std::fs;

pub struct Config{
    query: String,
    file_path: String,
}

impl Config {
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("Not enough arguments");
    //     }
    //     let query_str = args[1].clone();
    //     let file_path = args[2].clone();
    //     Config{query:query_str, file_path}
    // }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Not Enough arguments")
        }
        else {
            let query_str = args[1].clone();
            let file_path = args[2].clone();
            Ok(Config{query:query_str, file_path})
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);

    Ok(())
}