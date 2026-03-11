# 练习：编写测试

来源章节：`046-2-5-1-编写测试.md`

## 练习目标

练习以下概念：

- `assert!`
- `assert_eq!`
- 自定义失败信息
- `#[should_panic]`
- 返回 `Result` 的测试

## 要求

1. 阅读 `src/lib.rs` 中的 `Rectangle`、`greeting` 和 `Guess`
2. 理解 4 个不同风格的测试
3. 运行 `cargo test`
4. 确认所有测试通过

## 运行方式

```bash
cargo test
```

## 目标结果

```txt
running 4 tests
...
test result: ok. 4 passed; 0 failed
```

## 提示

- `assert!` 更适合布尔条件
- `assert_eq!` 更适合比较具体值
- `#[should_panic]` 可以验证“按预期 panic”
- 返回 `Result<(), E>` 的测试方便配合 `?`
