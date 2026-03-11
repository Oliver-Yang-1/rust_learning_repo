#[allow(dead_code)]
#[derive(Debug)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
}

#[allow(dead_code)]
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}

#[allow(dead_code)]
fn describe_status(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Todo => "task is waiting",
        TaskStatus::Doing => "task is in progress",
        TaskStatus::Done => "task is finished",
    }
}

#[allow(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}

fn main() {
    // 任务 1：匹配普通枚举成员。
    let status = TaskStatus::Doing;
    println!("status: {}", describe_status(status));

    // 任务 2：匹配带数据的枚举成员。
    let message = Message::Write(String::from("study enums"));
    match message {
        Message::Write(text) => println!("message: note = {}", text),
        Message::Quit => println!("message: quit"),
        Message::Move { x, y } => println!("move to: ({}, {})", x, y),
    }

    let movement = Message::Move { x: 3, y: 5 };
    match movement {
        Message::Move { x, y } => println!("move to: ({}, {})", x, y),
        Message::Write(text) => println!("message: note = {}", text),
        Message::Quit => println!("message: quit"),
    }

    // 任务 3：处理 Option。
    println!("plus one some: {:?}", plus_one(Some(5)));
    println!("plus one none: {:?}", plus_one(None));

    // 为了避免“未使用成员”的误解，这里显式创建一次 Quit。
    let _unused_quit = Message::Quit;

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
