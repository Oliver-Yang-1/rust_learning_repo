#[allow(dead_code)]
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

#[allow(dead_code)]
fn require_positive(n: i32) {
    if n <= 0 {
        panic!("n must be positive, got {}", n);
    }
}

fn main() {
    // 任务 1：可恢复错误用 Result 表示。
    match safe_divide(10, 2) {
        Ok(value) => println!("divide ok: {}", value),
        Err(error) => println!("divide err: {}", error),
    }

    // 任务 2：违反明显前置条件时可使用 panic!。
    // 你可以把 5 改成 0，再运行体验 panic。
    require_positive(5);
    println!("positive check passed");

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
