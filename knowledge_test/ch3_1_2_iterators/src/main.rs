use ch3_1_2_iterators::{Counter, Shoe, double_and_sum_even, shoes_in_size};

fn main() {
    // 任务 1：使用 iter/filter/map/sum，对偶数翻倍后求和，并打印：
    // sum of doubled evens: 24
    let numbers = vec![1, 2, 3, 4, 5, 6];
    println!("sum of doubled evens: {}", double_and_sum_even(&numbers));

    // 任务 2：使用 into_iter/filter/collect，只保留 10 码的鞋，并打印数量：
    // filtered shoes: 2
    let shoes = vec![
        Shoe::new(10, "sneaker"),
        Shoe::new(13, "sandal"),
        Shoe::new(10, "boot"),
    ];
    let filtered = shoes_in_size(shoes, 10);
    println!("filtered shoes: {}", filtered.len());

    // 任务 3：使用自定义 Counter 迭代器，复现书里的 zip/map/filter/sum 链式调用：
    // counter zip sum: 18
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|value| value % 3 == 0)
        .sum();
    println!("counter zip sum: {}", sum);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
