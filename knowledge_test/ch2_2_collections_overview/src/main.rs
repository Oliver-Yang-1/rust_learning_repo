use std::collections::HashMap;

fn main() {
    // 任务 1：使用 vector 保存一组分数。
    let scores = vec![80, 90, 100];
    let sum: i32 = scores.iter().sum();
    println!("score sum: {}", sum);

    // 任务 2：使用 String 拼接文本。
    let mut message = String::from("Rust");
    message.push(' ');
    message.push_str("collections");
    println!("message: {}", message);

    // 任务 3：使用 HashMap 保存键值对。
    let mut team_scores = HashMap::new();
    team_scores.insert(String::from("Blue"), 10);
    team_scores.insert(String::from("Yellow"), 50);
    println!("blue score: {}", team_scores.get("Blue").unwrap_or(&0));

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
