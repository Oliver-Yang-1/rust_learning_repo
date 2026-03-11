# 练习：循环与迭代器的性能比较

来源章节：`061-3-1-4-性能比较-循环对迭代器.md`

## 练习目标

练习以下概念：

- 用 `for` 循环实现搜索
- 用迭代器实现同样的搜索
- 验证两种实现结果一致
- 用 `Instant` 粗略观察两种实现的耗时

## 要求

1. 阅读 `src/lib.rs` 中 `search_for` 和 `search_iter` 两个版本
2. 运行程序，确认两种实现的匹配数量相同
3. 观察 `same results: true`
4. 比较 `cargo run` 和 `cargo run --release` 的耗时差异

## 运行方式

```bash
cargo run
cargo run --release
```

## 输出格式

实际耗时会变化，但输出格式应类似：

```txt
loop matches: 6000
iterator matches: 6000
same results: true
loop elapsed: 123.45µs
iterator elapsed: 118.37µs
```

## 提示

- 这里的耗时只是演示，不是严格基准测试
- 重点是理解“高级抽象不一定更慢”
- 如果想做更严谨的对比，可以后续再学习 benchmark 工具
- 可以运行 `cargo test` 先验证两种实现返回值完全一致
