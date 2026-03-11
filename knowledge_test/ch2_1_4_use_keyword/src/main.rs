mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }
    }
}

use crate::front_of_house::hosting;
use std::collections::HashMap;
use std::io::Result as IoResult;

fn alias_demo() -> IoResult<()> {
    Ok(())
}

fn main() {
    // 任务 1：通过 use 简化模块调用。
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // 任务 2：导入标准库类型并直接使用。
    let mut tables = HashMap::new();
    tables.insert(1, "Alice");
    println!("table 1: {}", tables.get(&1).unwrap_or(&"unknown"));

    // 任务 3：演示 as 别名。
    println!("alias ready: {}", alias_demo().is_ok());

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
