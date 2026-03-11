enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn main() {
    // 任务 1：只关心 Quarter 的情况。
    let quarter = Coin::Quarter(String::from("Alaska"));
    let normal_coin = Coin::Penny;
    let mut other_count = 0;

    if let Coin::Quarter(state) = quarter {
        println!("state quarter from {}", state);
    }

    // 任务 2：其余情况进入 else。
    if let Coin::Quarter(state) = normal_coin {
        println!("state quarter from {}", state);
    } else {
        other_count += 1;
    }

    // 任务 3：只在 Some(3) 时打印。
    let some_value = Some(3u8);
    if let Some(3) = some_value {
        println!("three");
    }

    println!("other count: {}", other_count);

    // 显式创建其他成员，帮助理解这个枚举还能表示别的硬币。
    let _unused_nickel = Coin::Nickel;
    let _unused_dime = Coin::Dime;

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
