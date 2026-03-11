use std::collections::HashMap;

fn main() {
    // 任务 1：创建并插入键值对。
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("blue score: {}", scores.get("Blue").unwrap_or(&0));

    // 任务 2：覆盖已有键的值。
    scores.insert(String::from("Blue"), 25);
    println!("blue replaced: {}", scores.get("Blue").unwrap_or(&0));

    // 任务 3：只在键不存在时插入默认值。
    scores.entry(String::from("Red")).or_insert(30);
    println!("red score: {}", scores.get("Red").unwrap_or(&0));

    // 任务 4：统计单词出现次数。
    let text = "hello world hello rust";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("hello count: {}", counts.get("hello").unwrap_or(&0));

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
