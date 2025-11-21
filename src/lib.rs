use std::error::Error;
use std::{fs, vec};

pub struct Config{
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>) 
        -> Result<Config, &'static str> {
            args.next();
            let query = match args.next(){
                Some(q) => q,
                None => return Err("Missing key word for query")
            };
            let file_path = match args.next(){
                Some(f) => f,
                None => return Err("Missing target file path")
            };

            Ok(Config { query, file_path })
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