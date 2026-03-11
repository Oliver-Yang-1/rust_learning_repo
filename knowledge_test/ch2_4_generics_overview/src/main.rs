#[allow(dead_code)]
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut current = list[0];
    for &item in list {
        if item > current {
            current = item;
        }
    }
    current
}

trait Label {
    fn label(&self) -> String {
        String::from("[item]")
    }
}

struct Book {
    #[allow(dead_code)]
    title: String,
}

impl Label for Book {}

#[allow(dead_code)]
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let numbers = vec![3, 9, 4, 1];
    println!("largest number: {}", largest(&numbers));

    let book = Book {
        title: String::from("Rust Book"),
    };
    println!("label: {}", book.label());

    let a = "rust";
    let b = "rustacean";
    println!("longer text: {}", longer(a, b));

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
