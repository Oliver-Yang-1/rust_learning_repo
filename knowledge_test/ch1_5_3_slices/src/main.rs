#[allow(dead_code)]
fn first_word(text: &str) -> &str {
    for (index, byte) in text.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            return &text[..index];
        }
    }

    text
}

fn main() {
    // 任务 1：对 String 调用 first_word。
    let message = String::from("hello world");
    let word = first_word(&message);
    println!("first word from String: {}", word);

    // 任务 2：对字符串字面值调用同一个函数。
    let literal = "rust slices";
    let literal_word = first_word(literal);
    println!("first word from literal: {}", literal_word);

    // 任务 3：打印整个字符串的 slice。
    let whole = &message[..];
    println!("whole slice: {}", whole);

    // 任务 4：创建数组 slice。
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4];
    println!("number slice: {:?}", middle);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
