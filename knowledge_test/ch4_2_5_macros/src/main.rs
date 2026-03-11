macro_rules! simple_vec {
    ( $( $x:expr ),* $(,)? ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

macro_rules! repeat_message {
    ($msg:expr; $count:expr) => {{
        let mut output = Vec::new();
        for _ in 0..$count {
            output.push($msg);
        }
        output
    }};
}

fn main() {
    // 任务 1：用声明宏接收可变数量参数。
    let numbers = simple_vec![1, 2, 3];
    println!("macro vec len: {}", numbers.len());

    // 任务 2：让宏生成重复代码。
    let messages = repeat_message!("rust"; 3);
    println!("macro repeat: {}", messages.join(", "));
}
