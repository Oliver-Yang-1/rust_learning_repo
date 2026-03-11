fn main() {
    // 任务 1：保留相同的业务逻辑，分别用 dev 和 release 配置构建。
    // 任务 2：观察 Cargo.toml 里的 [profile.dev] 和 [profile.release] 配置。
    // 任务 3：确认程序输出不变，但编译模式和性能倾向不同。
    let numbers = [1_u32, 2, 3, 4, 5];
    let weighted_sum: u32 = numbers
        .iter()
        .enumerate()
        .map(|(index, value)| (index as u32 + 1) * value)
        .sum();

    println!("weighted sum: {weighted_sum}");
    println!("build profile demo complete");
}
