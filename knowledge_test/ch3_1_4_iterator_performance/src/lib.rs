pub fn search_for<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{search_for, search_iter};

    #[test]
    fn loop_and_iterator_versions_return_same_results() {
        let contents = "\
Rust is fast.
Rust is safe.
This line does not match.
Trust the iterator.";

        assert_eq!(search_for("Rust", contents), search_iter("Rust", contents));
    }

    #[test]
    fn both_versions_find_two_lines() {
        let contents = "\
Rust is fast.
Rust is safe.
This line does not match.";

        assert_eq!(search_for("Rust", contents).len(), 2);
        assert_eq!(search_iter("Rust", contents).len(), 2);
    }
}
