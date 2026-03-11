mod front_of_house;

use crate::front_of_house::hosting;

fn main() {
    // 任务 1：通过跨文件模块调用函数。
    hosting::add_to_waitlist("Alice");
    hosting::add_to_waitlist("Bob");

    println!("service ready");

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
