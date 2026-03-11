use std::error::Error;
use std::num::ParseIntError;

#[allow(dead_code)]
fn parse_port(text: &str) -> Result<u16, ParseIntError> {
    let port = text.parse::<u16>()?;
    Ok(port)
}

#[allow(dead_code)]
fn describe_port(text: &str) -> Result<String, ParseIntError> {
    let port = parse_port(text)?;
    Ok(format!("server port is {}", port))
}

fn main() -> Result<(), Box<dyn Error>> {
    // 任务 1：使用 ? 传播可恢复错误。
    let port = parse_port("8080")?;
    println!("parsed port: {}", port);

    // 任务 2：main 也可以返回 Result。
    let description = describe_port("8080")?;
    println!("description: {}", description);

    Ok(())
}
