use std::env;
use std::process;

use ch2_6_3_refactor_config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = ch2_6_3_refactor_config::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
