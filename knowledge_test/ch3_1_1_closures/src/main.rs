use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
    computed_times: u32,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            values: HashMap::new(),
            computed_times: 0,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(value) = self.values.get(&arg) {
            return *value;
        }

        let value = (self.calculation)(arg);
        self.values.insert(arg, value);
        self.computed_times += 1;
        value
    }
}

fn main() {
    // 任务 1：定义一个闭包，把传入标签拼成 workout-xxx，并打印：
    // closure label: workout-pushups
    let format_label = |label: &str| format!("workout-{label}");
    println!("closure label: {}", format_label("pushups"));

    // 任务 2：定义一个捕获外部变量 limit 的闭包，判断数值是否不超过 limit，并打印：
    // captured limit: true
    let limit = 10;
    let within_limit = |value: u32| value <= limit;
    println!("captured limit: {}", within_limit(8));

    // 任务 3：定义一个 move 闭包，把 tags 移入闭包后再拼接输出：
    // move closure tags: rust, closure, move
    let tags = vec!["rust", "closure", "move"];
    let show_tags = move || tags.join(", ");
    println!("move closure tags: {}", show_tags());

    // 任务 4：使用带缓存的 Cacher，验证相同参数只计算一次。
    // 注意：10 被请求两次，20 被请求一次，所以真正计算只发生两次。
    let mut cacher = Cacher::new(|num| num * 2);
    println!("cached 10: {}", cacher.value(10));
    println!("cached 10 again: {}", cacher.value(10));
    println!("cached 20: {}", cacher.value(20));
    println!("computed times: {}", cacher.computed_times);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}

#[cfg(test)]
mod tests {
    use super::Cacher;

    #[test]
    fn cacher_reuses_value_for_same_argument() {
        let mut cacher = Cacher::new(|num| num * 2);

        assert_eq!(cacher.value(10), 20);
        assert_eq!(cacher.value(10), 20);
        assert_eq!(cacher.value(20), 40);
        assert_eq!(cacher.computed_times, 2);
    }
}
