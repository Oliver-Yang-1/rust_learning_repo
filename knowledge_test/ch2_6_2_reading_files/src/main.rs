use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    // 任务 1：读取整个文件到一个 String。
    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("searching for: {}", query);
    println!("in file: {}", filename);
    println!("with text:\n{}", contents);

    // 完成后可用 README 中的命令验证输出。
}
