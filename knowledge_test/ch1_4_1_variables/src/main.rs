#[allow(dead_code)]
const MAX_RETRY: u32 = 3;

fn main() {
    // 任务 1：定义不可变变量 x = 5，并打印：
    // immutable x: 5
    let x = 5;
    println!("immutable x: {}", x);

    // 任务 2：定义可变变量 counter = 0，把它改成 2，并打印：
    // mutable counter: 2
    let mut counter = 0;
    counter = 2;
    println!("mutable counter: {}", counter);

    // 任务 3：先定义 spaces = "   "，然后用隐藏把 spaces 变成长度 3，并打印：
    // shadowed spaces: 3
    let spaces = String::from("   ");
    let spaces = spaces.len();
    println!("shadowed spaces: {}", spaces);

    // 任务 4：打印常量：
    // max retry: 3
    println!("max retry: {}", MAX_RETRY);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
