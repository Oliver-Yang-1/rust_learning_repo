# 练习：自动化测试总览

来源章节：`045-2-5-测试.md`

## 练习目标

练习以下概念：

- `#[test]`
- `cargo test`
- 用断言验证代码行为
- 测试通过与失败的基本含义

## 要求

1. 阅读 `src/lib.rs` 中的函数和测试
2. 理解为什么测试模块使用 `#[cfg(test)]`
3. 运行 `cargo test`
4. 确认两个测试都通过

## 运行方式

```bash
cargo test
```

## 目标结果

```txt
running 2 tests
...
test result: ok. 2 passed; 0 failed
```

## 提示

- 测试代码通常写在 `#[cfg(test)] mod tests` 中
- `assert_eq!` 用来比较“实际值”和“期望值”
- 库项目比二进制项目更适合先练测试
