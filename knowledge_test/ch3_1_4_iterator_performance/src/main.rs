use std::time::{Duration, Instant};

use ch3_1_4_iterator_performance::{search_for, search_iter};

const SAMPLE: &str = "\
Rust makes systems programming safer.
Rust keeps performance predictable.
Iterators make Rust code expressive.
Loops can also be clear.
Trust the compiler.
";

fn main() {
    // 任务 1：准备一份稍大的文本，让循环版和迭代器版都在同一份数据上搜索。
    let contents = SAMPLE.repeat(2_000);

    // 任务 2：运行 for 循环版本。
    let (loop_matches, loop_elapsed) = run_benchmark("Rust", &contents, search_for);

    // 任务 3：运行迭代器版本。
    let (iterator_matches, iterator_elapsed) = run_benchmark("Rust", &contents, search_iter);

    // 任务 4：确认两种写法结果相同，再观察耗时。
    let same_results = search_for("Rust", &contents) == search_iter("Rust", &contents);

    println!("loop matches: {}", loop_matches);
    println!("iterator matches: {}", iterator_matches);
    println!("same results: {}", same_results);
    println!("loop elapsed: {:?}", loop_elapsed);
    println!("iterator elapsed: {:?}", iterator_elapsed);

    // 注意：耗时会随机器、负载和编译模式变化而变化。
    // 建议对比 `cargo run` 和 `cargo run --release` 的差异。
}

fn run_benchmark<F>(query: &str, contents: &str, search: F) -> (usize, Duration)
where
    F: for<'a> Fn(&str, &'a str) -> Vec<&'a str>,
{
    let start = Instant::now();
    let matches = search(query, contents);
    (matches.len(), start.elapsed())
}
