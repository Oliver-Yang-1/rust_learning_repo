pub struct UserId(u32);

impl UserId {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

pub type Kilometers = i32;
pub type Thunk = Box<dyn Fn() + Send + 'static>;

pub fn run_thunk(task: Thunk) -> &'static str {
    task();
    "thunk executed"
}

pub fn first_char<T: ?Sized + AsRef<str>>(value: &T) -> Option<char> {
    value.as_ref().chars().next()
}

pub fn parse_or_continue(input: &[&str]) -> Vec<u32> {
    let mut numbers = Vec::new();

    for item in input {
        let parsed: u32 = match item.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        numbers.push(parsed);
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::{Kilometers, UserId, first_char, parse_or_continue, run_thunk};

    #[test]
    fn newtype_can_provide_type_safe_api() {
        let user_id = UserId::new(42);
        assert_eq!(user_id.value(), 42);
    }

    #[test]
    fn type_alias_can_reduce_visual_noise() {
        let distance: Kilometers = 5;
        let base: i32 = 7;
        assert_eq!(distance + base, 12);
    }

    #[test]
    fn thunk_alias_can_be_used_as_function_parameter() {
        let message = run_thunk(Box::new(|| {}));
        assert_eq!(message, "thunk executed");
    }

    #[test]
    fn question_sized_allows_working_with_str() {
        assert_eq!(first_char("rust"), Some('r'));
    }

    #[test]
    fn continue_branch_allows_match_to_stay_u32() {
        let values = parse_or_continue(&["1", "bad", "3"]);
        assert_eq!(values, vec![1, 3]);
    }
}
