#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    // 任务 1：创建两个长方形。
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 80,
        height: 60,
    };

    // 任务 2：打印调试信息和面积。
    println!("rect1: {:?}", rect1);
    println!("rect1 area: {}", area(&rect1));
    println!("rect2: {:?}", rect2);
    println!("rect2 area: {}", area(&rect2));

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
