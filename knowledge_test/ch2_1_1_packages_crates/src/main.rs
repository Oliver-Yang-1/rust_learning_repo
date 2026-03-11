use ch2_1_1_packages_crates::{library_name, shared_message};

fn main() {
    // 任务 1：main.rs 代表当前包里的二进制 crate。
    println!("binary crate: running");

    // 任务 2：调用同一个包里 lib.rs 暴露出来的公共函数。
    println!("library crate: {}", library_name());
    println!("message: {}", shared_message());

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
