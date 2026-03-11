fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    // 任务 1：用原始标识符定义一个本来会和关键字冲突的函数名。
    println!("raw identifier works: {}", r#match("foo", "foobar"));

    // 任务 2：展示几个常见关键字在语法里的真实位置。
    let value = Some(5);
    match value {
        Some(number) if number > 3 => println!("keyword demo: match + if + let"),
        _ => println!("keyword demo: fallback"),
    }

    // 任务 3：演示 dyn trait 对象关键字。
    let formatter: Box<dyn Fn(i32) -> String> = Box::new(|x| format!("value = {x}"));
    println!("keyword dyn: {}", formatter(5));
}
