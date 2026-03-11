#[allow(dead_code)]
fn borrow_length(text: &String) -> usize {
    text.len()
}

#[allow(dead_code)]
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

fn main() {
    // 任务 1：创建一个 String，并把它移动给 moved_topic。
    // 注意：移动之后不要再使用原来的 topic。
    let topic = String::from("Rust ownership");
    let moved_topic = topic;

    // 任务 2：用 clone 再复制一份数据。
    let backup_topic = moved_topic.clone();

    // 任务 3：通过借用读取字符串长度。
    let length = borrow_length(&moved_topic);

    // 任务 4：展示 Copy 类型赋值后原变量仍然可用。
    let copied_number = 7;
    let another_number = copied_number;

    // 任务 5：获取第一个单词。
    let summary = String::from("ownership rules matter");
    let word = first_word(&summary);

    println!("moved topic: {}", moved_topic);
    println!("backup topic: {}", backup_topic);
    println!("borrowed length: {}", length);
    println!("copied numbers: {} and {}", copied_number, another_number);
    println!("first word: {}", word);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
