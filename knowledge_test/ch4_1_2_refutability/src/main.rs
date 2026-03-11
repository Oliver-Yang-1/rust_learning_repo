fn main() {
    // 任务 1：let 只能使用不可反驳模式。
    let number = 7;
    let value = number;
    println!("irrefutable let: {value}");

    // 任务 2：if let 适合使用可反驳模式。
    let maybe_name = Some("Rust");
    if let Some(name) = maybe_name {
        println!("refutable if let: {name}");
    }

    // 任务 3：while let 也依赖可反驳模式，不匹配时自动结束循环。
    let mut values = vec![10, 20];
    while let Some(item) = values.pop() {
        println!("refutable while let: {item}");
    }

    // 任务 4：match 的最后一个分支通常要负责兜底，体现穷尽性。
    let result: Result<u8, _> = "34".parse();
    match result {
        Ok(age) => println!("match exhaustive: age {age}"),
        Err(_) => println!("match exhaustive: parse failed"),
    }
}
