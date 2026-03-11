# 练习：用迭代器改进 I/O 项目

来源章节：`060-3-1-3-改进之前的-I-O-项目.md`

## 练习目标

练习以下概念：

- 让 `Config::new` 直接接收 `env::args()` 返回的迭代器
- 在 `main.rs` 中集中处理错误
- 用迭代器改写 `search`
- 用环境变量控制大小写敏感搜索

## 要求

1. 阅读 `src/main.rs` 和 `src/lib.rs`
2. 理解为什么 `main` 只负责参数解析失败和运行失败时的出口
3. 观察 `search` 如何从 `for` 循环改成 `lines/filter/collect`
4. 运行大小写敏感与不敏感两种搜索

## 运行方式

```bash
cargo run -- duct poem.txt
CASE_INSENSITIVE=1 cargo run -- rUsT poem.txt
```

## 目标输出

运行 `cargo run -- duct poem.txt` 时：

```txt
safe, fast, productive.
```

运行 `CASE_INSENSITIVE=1 cargo run -- rUsT poem.txt` 时：

```txt
Rust:
Trust me.
```

## 提示

- `env::args()` 的第一个值是程序名
- `main.rs` 越薄，越容易测试和定位错误
- `search` 用迭代器适配器后更接近“我要什么结果”而不是“如何循环”
- 可以再运行 `cargo test` 看搜索函数的单元测试
