use std::env;
use std::fs;

fn main() {
    // 任务 1：读取两个命令行参数。
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    // 任务 2：读取文件内容。
    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    // 任务 3：统计匹配的行数。
    let matched_lines = contents.lines().filter(|line| line.contains(query)).count();

    println!("query: {}", query);
    println!("file: {}", filename);
    println!("matched lines: {}", matched_lines);

    // 完成后可用 README 中的命令验证输出。
}
