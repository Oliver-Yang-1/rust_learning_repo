# 练习：将 crate 发布到 Crates.io

来源章节：`064-3-2-2-将-crate-发布到-Crates.io.md`

## 练习目标

练习以下概念：

- `Cargo.toml` 发布元信息
- `///` 文档注释
- `//!` crate 文档
- 文档测试
- `pub use` 重导出公有 API

## 要求

1. 阅读 `Cargo.toml` 中的 `description`、`license`、`repository`
2. 阅读 `src/lib.rs` 中的 `//!` 和 `///`
3. 运行 `cargo test`，确认普通测试和文档测试都通过
4. 运行 `cargo doc --no-deps` 生成文档
5. 观察 `PrimaryColor` 和 `mix` 能否直接从 crate 顶层导入

## 运行方式

```bash
cargo test
cargo doc --no-deps
```

## 提示

- 本练习不会真的执行 `cargo publish`
- 发布前最重要的是先补齐元信息和文档
- `pub use` 可以让外部用户不必关心你的内部模块层级
