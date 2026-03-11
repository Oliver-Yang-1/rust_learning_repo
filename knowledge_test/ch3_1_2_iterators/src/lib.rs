#[derive(Debug, PartialEq, Eq)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

impl Shoe {
    pub fn new(size: u32, style: &str) -> Self {
        Self {
            size,
            style: style.to_string(),
        }
    }
}

pub fn double_and_sum_even(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|number| *number % 2 == 0)
        .map(|number| number * 2)
        .sum()
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Counter, Shoe, double_and_sum_even, shoes_in_size};

    #[test]
    fn double_and_sum_even_numbers() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(double_and_sum_even(&numbers), 24);
    }

    #[test]
    fn filter_shoes_by_size() {
        let shoes = vec![
            Shoe::new(10, "sneaker"),
            Shoe::new(13, "sandal"),
            Shoe::new(10, "boot"),
        ];

        assert_eq!(
            shoes_in_size(shoes, 10),
            vec![Shoe::new(10, "sneaker"), Shoe::new(10, "boot")]
        );
    }

    #[test]
    fn counter_next_returns_one_to_five() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn counter_iterator_pipeline_matches_book_example() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|value| value % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}
