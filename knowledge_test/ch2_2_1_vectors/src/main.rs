fn main() {
    // 任务 1：创建空 vector，并添加元素。
    let mut values: Vec<i32> = Vec::new();
    values.push(10);
    values.push(20);
    values.push(30);

    // 任务 2：安全读取第二个元素。
    match values.get(1) {
        Some(item) => println!("second item: {}", item),
        None => println!("second item: missing"),
    }

    // 任务 3：遍历并计算总和。
    let mut sum = 0;
    for item in &values {
        sum += item;
    }
    println!("sum: {}", sum);

    // 任务 4：通过可变引用批量修改元素。
    for item in &mut values {
        *item += 5;
    }
    println!("updated vector: {:?}", values);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
