use std::thread;

fn main() {
    // 任务 1：在主线程里准备一份数据。
    let numbers = vec![1, 2, 3];
    println!("main thread: preparing data");

    // 任务 2：使用 move 闭包把数据移动到新线程。
    let handle = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        format!("spawned thread sum: {sum}")
    });

    // 任务 3：在主线程中等待新线程结束，并拿到返回值。
    let message = handle.join().unwrap();
    println!("{message}");
    println!("main thread: finished");

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
