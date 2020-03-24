use std::env;
use std::process;

use minigrep::run;
use minigrep::search;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    // dont care about the Ok Result here so we can just use if let Err(e)
    if let Err(e) = run(config) {
        println!("Error occured while running: {}", e);
        process::exit(1)
    }
}
