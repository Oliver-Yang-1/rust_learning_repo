pub fn add_two(value: i32) -> i32 {
    value + 2
}

pub fn prints_and_returns_10(value: i32) -> i32 {
    println!("I got the value {}", value);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn print_example() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 真实项目里，这里可能是很慢的测试。
        assert_eq!(102, add_two(100));
    }
}
