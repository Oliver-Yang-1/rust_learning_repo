pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::add_one;

    #[test]
    fn adds_one() {
        assert_eq!(add_one(2), 3);
    }
}
