fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("function params: ({x}, {y})");
}

fn main() {
    // 任务 1：在 let 语句里解构元组。
    let (x, y, z) = (1, 2, 3);
    println!("let pattern: {x}, {y}, {z}");

    // 任务 2：在 for 循环里用模式解构 enumerate 产生的元组。
    let letters = ['a', 'b'];
    for (index, value) in letters.iter().enumerate() {
        println!("for pattern: {value} at {index}");
    }

    // 任务 3：在函数参数里用模式解构引用元组。
    let point = (3, 5);
    print_coordinates(&point);

    // 任务 4：在 if let 里匹配 Option。
    let favorite_color = Some("green");
    if let Some(color) = favorite_color {
        println!("if let pattern: {color}");
    }

    // 任务 5：在 while let 里把栈顶元素依次弹出。
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("while let pattern: {top}");
    }

    // 任务 6：在 match 分支里匹配不同情况。
    let value = Some(5);
    match value {
        Some(5) => println!("match pattern: got five"),
        Some(number) => println!("match pattern: got {number}"),
        None => println!("match pattern: got none"),
    }
}
