# 练习：变量与可变性

来源章节：`009-1-4-1-变量与可变性.md`

## 练习目标

练习以下概念：

- 不可变变量
- `mut` 可变变量
- `const` 常量
- 隐藏（shadowing）

## 要求

1. 定义一个不可变变量 `x = 5`，并打印
2. 定义一个可变变量 `counter = 0`，修改到 `2` 后打印
3. 定义 `spaces = "   "`，再用隐藏把它变成长度
4. 定义常量 `MAX_RETRY: u32 = 3`

## 目标输出

```txt
immutable x: 5
mutable counter: 2
shadowed spaces: 3
max retry: 3
```

## 提示

- 常量要写类型
- `mut` 是修改同一个变量
- 隐藏是重新 `let` 一个同名新变量
