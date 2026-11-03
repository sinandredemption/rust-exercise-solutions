use core::panic;
use std::env;
use minigrep::Config;

fn main() {

    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        panic!("parsing fn said '{err}' :(");
    });


    if let Err(e) = minigrep::run(config) {
        println!("error >< : '{e}'");
       std::process::exit(1);
    }
}



