enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {}", state);
            25
        }
    }
}

#[allow(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}

#[allow(dead_code)]
fn describe_number(x: u8) -> &'static str {
    match x {
        1 => "one",
        3 => "three",
        5 => "five",
        _ => "other",
    }
}

fn main() {
    println!("coin value: {}", value_in_cents(Coin::Penny));
    println!(
        "coin value: {}",
        value_in_cents(Coin::Quarter(String::from("Alaska")))
    );
    println!("plus one: {:?}", plus_one(Some(10)));
    println!("number: {}", describe_number(3));

    // 显式创建其他成员，便于对照题目理解。
    let _unused_nickel = Coin::Nickel;
    let _unused_dime = Coin::Dime;

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
