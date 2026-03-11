use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Task {
    pub priority: u8,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn demo_task() -> Task {
    Task {
        priority: 2,
        title: String::from("read appendix C"),
        done: false,
    }
}

pub fn collect_sorted_titles(tasks: &[Task]) -> Vec<String> {
    let ordered: BTreeSet<Task> = tasks.iter().cloned().collect();
    ordered.into_iter().map(|task| task.title).collect()
}

pub fn count_by_task(tasks: &[Task]) -> HashMap<Task, usize> {
    let mut counts = HashMap::new();
    for task in tasks {
        *counts.entry(task.clone()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_and_partial_eq_help_assertions() {
        let left = demo_task();
        let right = demo_task();
        assert_eq!(left, right);
    }

    #[test]
    fn ord_allows_btree_set_sorting() {
        let tasks = vec![
            Task {
                priority: 3,
                title: String::from("write notes"),
                done: false,
            },
            Task {
                priority: 1,
                title: String::from("borrow checker review"),
                done: false,
            },
            Task {
                priority: 2,
                title: String::from("traits recap"),
                done: true,
            },
        ];

        let titles = collect_sorted_titles(&tasks);
        assert_eq!(
            titles,
            vec![
                String::from("borrow checker review"),
                String::from("traits recap"),
                String::from("write notes"),
            ]
        );
    }

    #[test]
    fn hash_and_eq_allow_hash_map_keys() {
        let repeated = demo_task();
        let counts = count_by_task(&[repeated.clone(), repeated]);
        assert_eq!(counts.values().copied().sum::<usize>(), 2);
    }

    #[test]
    fn default_builds_an_empty_task() {
        let task = Task::default();
        assert_eq!(task.priority, 0);
        assert_eq!(task.title, "");
        assert!(!task.done);
    }

    #[test]
    fn copy_duplicates_stack_values_without_clone_call() {
        let start = Point { x: 3, y: 4 };
        let copied = start;
        assert_eq!(start, copied);
    }
}
