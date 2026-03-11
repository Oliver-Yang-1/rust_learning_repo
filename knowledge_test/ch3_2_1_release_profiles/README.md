# 练习：采用发布配置自定义构建

来源章节：`063-3-2-1-采用发布配置自定义构建.md`

## 练习目标

练习以下概念：

- `dev` 配置
- `release` 配置
- `Cargo.toml` 中的 `[profile.*]`
- `opt-level`

## 要求

1. 阅读 `Cargo.toml` 中的 `[profile.dev]` 和 `[profile.release]`
2. 运行 `cargo build`
3. 运行 `cargo build --release`
4. 运行程序并确认两种构建方式的输出一致

## 运行方式

```bash
cargo build
cargo run

cargo build --release
cargo run --release
```

## 目标输出

```txt
weighted sum: 55
build profile demo complete
```

## 提示

- `dev` 更偏向开发体验，通常编译更快
- `release` 更偏向运行性能，通常优化更多
- 这类配置影响编译方式，不影响程序逻辑结果
