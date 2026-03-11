use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 任务 1：使用 Arc<Mutex<i32>> 创建共享计数器。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // 任务 2：启动 10 个线程，每个线程把计数器加一。
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 任务 3：等待所有线程结束后再读取结果。
    for handle in handles {
        handle.join().unwrap();
    }

    println!("final counter: {}", *counter.lock().unwrap());
}
