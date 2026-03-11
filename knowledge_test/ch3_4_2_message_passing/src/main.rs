use std::sync::mpsc;
use std::thread;

fn main() {
    // 任务 1：创建一个 channel。
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    // 任务 2：启动两个生产者线程，各自发送消息。
    let producer_a = thread::spawn(move || {
        tx.send(String::from("alpha")).unwrap();
        tx.send(String::from("beta")).unwrap();
    });

    let producer_b = thread::spawn(move || {
        tx_clone.send(String::from("gamma")).unwrap();
        tx_clone.send(String::from("delta")).unwrap();
    });

    producer_a.join().unwrap();
    producer_b.join().unwrap();

    // 两个发送端都在线程结束后被丢弃，此时接收端迭代会自然结束。
    let mut received = Vec::new();
    for message in rx {
        received.push(message);
    }

    received.sort();

    println!("received count: {}", received.len());
    println!("received messages: {}", received.join(", "));

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
