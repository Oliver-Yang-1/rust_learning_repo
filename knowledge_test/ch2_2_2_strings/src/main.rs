fn main() {
    // 任务 1：创建并更新 String。
    let mut message = String::from("Ru");
    message.push_str("st");
    message.push('!');
    println!("message: {}", message);

    // 任务 2：用 format! 拼接字符串，不移动原值。
    let combined = format!("{} {}", message, "language");
    println!("combined: {}", combined);

    // 任务 3：创建字符串 slice。
    let text = String::from("hello world");
    let hello = &text[..5];
    println!("slice: {}", hello);

    // 任务 4：使用 chars() 正确统计 UTF-8 字符数量。
    let greeting = "你好";
    let char_count = greeting.chars().count();
    println!("char count: {}", char_count);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
