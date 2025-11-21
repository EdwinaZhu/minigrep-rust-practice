use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    // let query_str = &args[1];
    // let file_path = &args[2];
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Issue when parsing arguments:{err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
