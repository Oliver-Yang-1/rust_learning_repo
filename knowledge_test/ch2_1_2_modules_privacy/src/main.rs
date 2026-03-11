mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn fruit_name(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    #[allow(dead_code)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    // 任务 1：调用公有模块中的公有函数。
    front_of_house::hosting::add_to_waitlist();

    // 任务 2：通过公共关联函数创建包含私有字段的结构体。
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("toast: {}", meal.toast);

    // 任务 3：公有枚举的成员可以直接使用。
    let appetizer = back_of_house::Appetizer::Soup;
    match appetizer {
        back_of_house::Appetizer::Soup => println!("appetizer: Soup"),
        back_of_house::Appetizer::Salad => println!("appetizer: Salad"),
    }

    // 为了表明私有字段仍然存在，这里只通过公有方法读取，不直接访问字段。
    let _fruit = meal.fruit_name();

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
