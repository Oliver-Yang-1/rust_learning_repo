# 练习：Cargo 工作空间

来源章节：`065-3-2-3-Cargo-工作空间.md`

## 练习目标

练习以下概念：

- `[workspace]`
- 多个成员 crate
- 路径依赖
- 共享 `Cargo.lock`
- 在 workspace 根目录运行 `cargo build`、`cargo run -p`、`cargo test`

## 要求

1. 阅读根目录 `Cargo.toml`
2. 阅读 `adder`、`add-one`、`add-two` 三个成员 crate
3. 在 workspace 根目录运行 `cargo build`
4. 在 workspace 根目录运行 `cargo run -p adder`
5. 在 workspace 根目录运行 `cargo test`

## 目标输出

运行 `cargo run -p adder` 时：

```txt
10 plus one is 11
10 plus two is 12
```

## 提示

- workspace 根目录本身不是普通 package
- `adder` 依赖 `add-one` 和 `add-two`
- 在根目录运行命令时，Cargo 会统一管理成员 crate 的构建与测试
