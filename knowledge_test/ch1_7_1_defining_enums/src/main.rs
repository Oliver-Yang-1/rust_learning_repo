enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &'static str {
        "message handled"
    }
}

fn main() {
    // 任务 1：创建两个不同成员的 IP 地址枚举值。
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    match home {
        IpAddr::V4(a, b, c, d) => println!("home ipv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(text) => println!("home ipv6: {}", text),
    }

    match loopback {
        IpAddr::V4(a, b, c, d) => println!("loopback ipv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(text) => println!("loopback ipv6: {}", text),
    }

    // 任务 2：给枚举调用方法。
    let message = Message::Write(String::from("hello"));
    println!("message call: {}", message.call());

    // 任务 3：处理 Option。
    let optional_text = Some("rust");
    match optional_text {
        Some(text) => println!("optional text: {}", text),
        None => println!("optional text: empty"),
    }

    // 显式创建其他成员，方便理解一个枚举可以有不同形态。
    let _unused_quit = Message::Quit;
    let _unused_color = Message::ChangeColor(255, 0, 0);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
