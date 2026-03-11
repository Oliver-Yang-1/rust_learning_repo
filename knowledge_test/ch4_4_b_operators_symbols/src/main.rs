fn main() {
    // 任务 1：算术与赋值运算符。
    let mut total = 10;
    total += 5;
    println!("operator += : {total}");

    // 任务 2：范围与切片语法。
    let numbers = [1, 2, 3, 4, 5];
    println!("range slice: {:?}", &numbers[1..4]);

    // 任务 3：逻辑运算和比较。
    let comparison = total > 10 && total < 20;
    println!("logic compare: {comparison}");

    // 任务 4：turbofish 与路径符号。
    let parsed = "42".parse::<i32>().unwrap();
    println!("turbofish parse: {parsed}");

    // 任务 5：用问号传播错误的等价结果展示。
    println!("question mark demo: {}", parse_and_add_one("9").unwrap());
}

fn parse_and_add_one(input: &str) -> Result<i32, std::num::ParseIntError> {
    let value = input.parse::<i32>()?;
    Ok(value + 1)
}
