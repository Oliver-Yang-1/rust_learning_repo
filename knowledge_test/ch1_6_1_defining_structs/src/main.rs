struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // 任务 1：创建可变结构体实例，并修改字段。
    let mut user1 = build_user(
        String::from("ferris@rust.dev"),
        String::from("ferris"),
    );
    user1.sign_in_count = 2;
    user1.email = String::from("ferris_new@rust.dev");

    println!(
        "user1: {} / {} / {} / {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    // 任务 2：使用结构体更新语法创建 user2。
    let user2 = User {
        username: String::from("crab"),
        email: String::from("crab@rust.dev"),
        ..user1
    };

    println!(
        "user2: {} / {} / {} / {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
