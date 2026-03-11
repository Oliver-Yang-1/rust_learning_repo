# 练习：关键字与原始标识符

来源附录：

- 附录 A：关键字

## 练习目标

练习以下概念：

- Rust 关键字
- 原始标识符 `r#...`
- `dyn`
- `match`

## 要求

1. 用 `r#match` 定义一个与关键字同名的函数
2. 在 `main` 中调用这个函数
3. 再顺手观察 `match`、`if`、`let`、`dyn` 在真实代码中的位置

## 目标输出

```txt
raw identifier works: true
keyword demo: match + if + let
keyword dyn: value = 5
```

## 提示

- `r#` 前缀不会改变语义，只是让你能使用本来会冲突的关键字名字
- 这类写法在跨 edition 使用旧库 API 时尤其有用
