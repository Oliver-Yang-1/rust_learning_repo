mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }
    }
}

mod back_of_house {
    pub fn prepare() {
        println!("kitchen ready");
    }
}

use crate::front_of_house::hosting;

fn main() {
    // 任务 1：把对外暴露的函数放进模块中。
    println!("theme: packages crates modules");
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // 任务 2：内部模块也可以组织实现细节。
    back_of_house::prepare();

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
