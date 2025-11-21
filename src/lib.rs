use std::error::Error;
use std::{fs, vec};

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
    let results = search(&config.query, &contents);
    for line in results{
        println!("The lines are:\n{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut res:Vec<&str> = vec![];
    for l in contents.lines(){
        if l.contains(query){
            res.push(l);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.";

        assert_eq!(vec!["safe, fast, productive."], super::search(query, contents));
    }
}