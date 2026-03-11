struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping: {}", self.data);
    }
}

fn main() {
    // 任务 1：定义实现了 Drop 的类型。
    let early = CustomSmartPointer {
        data: String::from("early resource"),
    };

    // 任务 2：使用 std::mem::drop 提前释放资源。
    println!("before early drop");
    std::mem::drop(early);
    println!("after early drop");

    // 任务 3：让另一个值在作用域结束时自动 drop。
    let normal = CustomSmartPointer {
        data: String::from("normal resource"),
    };
    println!("before scope end: {}", normal.data);

    // 注意：`normal` 的 drop 输出会在 main 结束时自动出现。
}
