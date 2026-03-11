use std::env;
use std::process;

use ch3_1_3_improve_io_project::{Config, run};

fn main() {
    // 任务 1：把参数解析交给 Config::new，让 main 保持精简。
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 任务 2：把真正的 I/O 和搜索逻辑交给 run。
    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
