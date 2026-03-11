use std::env;
use std::process;

use ch2_6_6_stderr::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 错误信息应该写到标准错误，而不是标准输出。
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = ch2_6_6_stderr::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
