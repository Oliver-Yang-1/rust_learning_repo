#[allow(dead_code)]
fn calculate_length(text: &String) -> usize {
    text.len()
}

#[allow(dead_code)]
fn append_world(text: &mut String) {
    text.push_str(", world");
}

fn main() {
    let mut text = String::from("hello");

    // 任务 1：通过不可变引用读取长度。
    let length = calculate_length(&text);
    println!("length before change: {}", length);

    // 任务 2：使用单独作用域创建可变引用并修改字符串。
    {
        let text_ref = &mut text;
        append_world(text_ref);
    }

    println!("after change: {}", text);

    // 任务 3：修改结束后，再创建多个不可变引用。
    let first_borrow = &text;
    let second_borrow = &text;
    println!("shared borrows: {} | {}", first_borrow, second_borrow);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
