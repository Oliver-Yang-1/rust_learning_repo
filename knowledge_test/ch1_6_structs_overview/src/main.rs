struct Book {
    title: String,
    author: String,
    pages: u32,
    published: bool,
}

struct Rgb(u8, u8, u8);

#[allow(dead_code)]
fn build_book(title: String, author: String) -> Book {
    Book {
        title,
        author,
        pages: 120,
        published: true,
    }
}

fn main() {
    // 任务 1：使用普通结构体保存一本书的信息。
    let book1 = build_book(String::from("Rust Notes"), String::from("Ferris"));
    println!("book1: {} by {}", book1.title, book1.author);
    println!("book1 pages: {}", book1.pages);

    // 任务 2：使用结构体更新语法创建第二本书。
    let book2 = Book {
        title: String::from("Rust Notes Advanced"),
        ..book1
    };
    println!("book2: {} by {}", book2.title, book2.author);
    println!("book2 published: {}", book2.published);

    // 任务 3：使用元组结构体保存颜色。
    let accent = Rgb(30, 144, 255);
    println!("accent color: {}, {}, {}", accent.0, accent.1, accent.2);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
