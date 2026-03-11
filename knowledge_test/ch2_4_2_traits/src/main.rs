trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Tweet {
    username: String,
    #[allow(dead_code)]
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

#[allow(dead_code)]
fn notify<T: Summary>(item: &T) {
    println!("breaking news: {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("ferris"),
        content: String::from("learning traits"),
    };

    println!("summary: {}", tweet.summarize());
    notify(&tweet);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
