pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // 任务 1：用自定义类型封装“必须合法”的数据。
    let guess = Guess::new(42);
    println!("validated guess: {}", guess.value());

    // 你可以把 42 改成 142，观察因为违反契约而触发的 panic。
}
