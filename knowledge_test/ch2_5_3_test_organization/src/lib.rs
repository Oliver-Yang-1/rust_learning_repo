pub fn add_two(value: i32) -> i32 {
    internal_adder(value, 2)
}

fn internal_adder(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_uses_internal_adder() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn internal_function_can_be_tested() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
