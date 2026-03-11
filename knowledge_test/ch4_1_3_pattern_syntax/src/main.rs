#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    // 任务 1：匹配字面值和范围。
    let x = 5;
    match x {
        1..=5 => println!("range pattern: one through five"),
        _ => println!("range pattern: something else"),
    }

    // 任务 2：解构结构体。
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x, y: 0 } => println!("struct pattern: on x axis at {x}"),
        Point { x: 0, y } => println!("struct pattern: on y axis at {y}"),
        Point { x, y } => println!("struct pattern: point ({x}, {y})"),
    }

    // 任务 3：使用 _ 忽略部分值。
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("ignore pattern: {first}, {third}, {fifth}");
        }
    }

    // 任务 4：使用 .. 忽略剩余值。
    let tuple = (2, 4, 8, 16, 32);
    match tuple {
        (first, .., last) => println!("dotdot pattern: {first}, {last}"),
    }

    // 任务 5：使用匹配守卫。
    let num = Some(4);
    match num {
        Some(value) if value < 5 => println!("guard pattern: less than five"),
        Some(value) => println!("guard pattern: {value}"),
        None => println!("guard pattern: none"),
    }

    // 任务 6：使用 @ 绑定在测试范围的同时保留值。
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("at pattern: {id_variable}"),
        Message::Hello { id } => println!("at pattern: other id {id}"),
    }
}
