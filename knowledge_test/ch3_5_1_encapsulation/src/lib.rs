pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        if result.is_some() {
            self.update_average();
        }

        result
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    fn update_average(&mut self) {
        if self.list.is_empty() {
            self.average = 0.0;
            return;
        }

        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::AveragedCollection;

    #[test]
    fn add_keeps_average_in_sync() {
        let mut numbers = AveragedCollection::new();

        numbers.add(10);
        numbers.add(20);
        numbers.add(30);

        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers.average(), 20.0);
    }

    #[test]
    fn remove_updates_cached_average() {
        let mut numbers = AveragedCollection::new();
        numbers.add(10);
        numbers.add(20);
        numbers.add(30);

        assert_eq!(numbers.remove(), Some(30));
        assert_eq!(numbers.average(), 15.0);
    }

    #[test]
    fn removing_last_value_resets_average() {
        let mut numbers = AveragedCollection::new();
        numbers.add(42);

        assert_eq!(numbers.remove(), Some(42));
        assert_eq!(numbers.average(), 0.0);
        assert_eq!(numbers.remove(), None);
    }
}
