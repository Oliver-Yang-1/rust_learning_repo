use std::env;

fn main() {
    // 任务 1：读取并收集命令行参数。
    let args: Vec<String> = env::args().collect();

    // 任务 2：保存查询词和文件名。
    let query = &args[1];
    let filename = &args[2];

    println!("arg count: {}", args.len());
    println!("searching for: {}", query);
    println!("in file: {}", filename);

    // 完成后可用 README 中的命令验证输出。
}
