# 练习：panic! 与不可恢复错误

来源章节：`038-2-3-1-panic!-与不可恢复的错误.md`

## 练习目标

练习以下概念：

- `panic!`
- 越界访问为何会 panic
- `get` 与索引的区别
- `RUST_BACKTRACE=1` 的用途

## 要求

1. 创建一个 `Vec<i32>`，内容为 `1, 2, 3`
2. 用 `get(1)` 安全读取第二个元素
3. 写一个函数 `must_have_index(values: &[i32], index: usize) -> i32`
4. 当索引超出范围时在函数中 `panic!`
5. 在 `main` 中先传入合法索引 `2`
6. 注释里说明如何改成 `99` 来观察 panic 和 backtrace

## 目标输出

```txt
safe read: 2
strict read: 3
```

## 提示

- `values.get(index)` 返回 `Option<&i32>`
- 可以用 `match` 在 `None` 时手动 `panic!`
- 想看更详细的调用栈可运行：
  `RUST_BACKTRACE=1 cargo run`
