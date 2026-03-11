mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist_by_absolute() {
            println!("added by absolute path");
        }

        pub fn add_to_waitlist_by_relative() {
            println!("added by relative path");
        }
    }
}

fn serve_order() {
    println!("order served");
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        super::serve_order();
    }
}

fn main() {
    // 任务 1：用绝对路径调用模块中的函数。
    crate::front_of_house::hosting::add_to_waitlist_by_absolute();

    // 任务 2：用相对路径调用同一模块中的函数。
    front_of_house::hosting::add_to_waitlist_by_relative();

    // 任务 3：在子模块中使用 super 调回父模块。
    back_of_house::fix_incorrect_order();

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
