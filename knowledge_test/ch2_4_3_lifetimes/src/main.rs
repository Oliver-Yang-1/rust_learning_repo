#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, _announcement: &str) -> &str {
        self.part
    }
}

fn main() {
    let a = String::from("abcd");
    let b = "xyz";
    println!("longest: {}", longest(a.as_str(), b));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split(". ")
        .next()
        .expect("Could not find a sentence.");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("level: {}", excerpt.level());
    println!(
        "excerpt: {}",
        excerpt.announce_and_return_part("pay attention")
    );

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
