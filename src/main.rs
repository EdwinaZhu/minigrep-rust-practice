use core::panic;
use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let query_str = &args[1];
    // let file_path = &args[2];
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Issue when parsing arguments:{err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config){
        println!("Application error: {e}");
        process::exit(1);
    };
}
