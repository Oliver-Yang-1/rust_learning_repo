#[allow(dead_code)]
fn build_message() -> String {
    String::from("hello")
}

fn main() {
    // 任务 1：在内部作用域中创建并打印一个 String。
    {
        let message = build_message();
        println!("message in scope: {}", message);
    }

    // 任务 2：把 owner1 移动给 owner2。
    let owner1 = String::from("Ferris");
    let owner2 = owner1;
    println!("moved owner: {}", owner2);

    // 任务 3：基于 owner2 克隆出 owner3。
    let owner3 = owner2.clone();
    println!("cloned owners: {} / {}", owner2, owner3);

    // 任务 4：整数实现了 Copy，赋值后两个变量都能继续使用。
    let x = 42;
    let y = x;
    println!("copy values: {} and {}", x, y);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
