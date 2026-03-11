use std::sync::{Arc, Mutex};
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    // 任务 1：用编译期断言验证 Arc<Mutex<i32>> 同时满足 Send 和 Sync。
    assert_send::<Arc<Mutex<i32>>>();
    assert_sync::<Arc<Mutex<i32>>>();

    // 任务 2：把 Arc<Mutex<i32>> 移动到线程中，证明它能安全跨线程共享。
    let shared = Arc::new(Mutex::new(41));
    let worker_shared = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        let mut value = worker_shared.lock().unwrap();
        *value += 1;
    });

    handle.join().unwrap();

    println!("send + sync check passed");
    println!("shared value after thread: {}", *shared.lock().unwrap());

    // 说明：
    // - 如果尝试把 Rc<Mutex<i32>> 放进线程，这里会直接编译失败。
    // - 这正是 Send/Sync 在编译期提供的保护。
}
