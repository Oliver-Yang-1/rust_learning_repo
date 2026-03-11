fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // 任务 1：使用函数指针作为参数。
    let answer = do_twice(add_one, 5);
    println!("function pointer answer: {answer}");

    // 任务 2：把函数名当作 map 的参数。
    let numbers = vec![1, 2, 3];
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("mapped strings: {}", strings.join(", "));

    // 任务 3：返回一个装箱闭包。
    let closure = returns_closure();
    println!("returned closure answer: {}", closure(5));
}
