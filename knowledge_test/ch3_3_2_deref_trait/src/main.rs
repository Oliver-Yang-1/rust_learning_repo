use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    // 任务 1：实现 MyBox<T> 和 Deref trait。
    let x = 5;
    let y = MyBox::new(x);
    println!("deref number: {}", *y);

    // 任务 2：观察 deref coercion，直接把 &MyBox<String> 传给接收 &str 的函数。
    let name = MyBox::new(String::from("Rust"));
    println!("deref greeting: {}", hello(&name));

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}

#[cfg(test)]
mod tests {
    use super::{MyBox, hello};

    #[test]
    fn mybox_can_be_dereferenced() {
        let value = MyBox::new(5);
        assert_eq!(*value, 5);
    }

    #[test]
    fn deref_coercion_turns_mybox_string_into_str() {
        let name = MyBox::new(String::from("Rust"));
        assert_eq!(hello(&name), "Hello, Rust!");
    }
}
