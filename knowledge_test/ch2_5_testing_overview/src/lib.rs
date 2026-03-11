pub fn add_two(value: i32) -> i32 {
    value + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_to_small_number() {
        // 最基本的测试结构：
        // 1. 准备输入
        // 2. 调用被测试代码
        // 3. 断言结果
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn adds_two_to_large_number() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}
