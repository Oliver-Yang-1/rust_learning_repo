# 练习：Refutability

来源章节：`087-4-1-2-Refutability-何时模式可能会匹配失败.md`

## 练习目标

练习以下概念：

- 不可反驳模式
- 可反驳模式
- `let` 与 `if let` 的适用场景
- `while let`
- `match` 的穷尽性

## 要求

1. 用 `let` 写一个一定能匹配成功的不可反驳模式
2. 用 `if let` 匹配 `Some(...)`
3. 用 `while let` 匹配 `Vec::pop()`
4. 用 `match` 处理 `Result` 的所有分支

## 目标输出

```txt
irrefutable let: 7
refutable if let: Rust
refutable while let: 20
refutable while let: 10
match exhaustive: age 34
```

## 提示

- `let Some(x) = ...` 这类写法之所以不行，是因为 `let` 要求模式必定匹配
- 如果模式可能失败，通常就该考虑 `if let`、`while let` 或 `match`
