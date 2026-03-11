fn main() {
    let num = 10;

    // 任务 1：调用 workspace 内部的两个库 crate。
    println!("{} plus one is {}", num, add_one::add_one(num));
    println!("{} plus two is {}", num, add_two::add_two(num));
}
