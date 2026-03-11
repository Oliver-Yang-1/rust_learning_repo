enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn sum(&self) -> i32 {
        match self {
            List::Cons(value, next) => value + next.sum(),
            List::Nil => 0,
        }
    }
}

fn main() {
    // 任务 1：把单个整数放到 Box<T> 中，并像普通值一样打印它。
    let boxed_number = Box::new(5);
    println!("boxed number: {}", boxed_number);

    // 任务 2：使用 Box<T> 定义递归类型，构造 1 -> 2 -> 3 的链表。
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    // 任务 3：递归遍历链表并求和。
    println!("list sum: {}", list.sum());

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
