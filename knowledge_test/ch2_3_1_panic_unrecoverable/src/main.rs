#[allow(dead_code)]
fn must_have_index(values: &[i32], index: usize) -> i32 {
    match values.get(index) {
        Some(value) => *value,
        None => panic!("index out of bounds: len is {}, index is {}", values.len(), index),
    }
}

fn main() {
    let values = vec![1, 2, 3];

    // 任务 1：安全读取可以使用 get。
    println!("safe read: {}", values.get(1).unwrap_or(&0));

    // 任务 2：严格要求索引存在时，可以选择 panic!。
    // 你可以把这里的 2 改成 99，然后运行：
    // RUST_BACKTRACE=1 cargo run
    println!("strict read: {}", must_have_index(&values, 2));

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
