# 练习：所有可能会用到模式的位置

来源章节：`086-4-1-1-所有可能会用到模式的位置.md`

## 练习目标

练习以下概念：

- `let` 中的模式
- `for` 中的模式
- 函数参数中的模式
- `if let`
- `while let`
- `match`

## 要求

1. 在 `let` 里解构元组
2. 在 `for` 循环里解构 `(index, value)`
3. 在函数参数里解构 `&(i32, i32)`
4. 用 `if let` 匹配 `Option`
5. 用 `while let` 逐个弹出栈元素
6. 用 `match` 覆盖所有情况

## 目标输出

```txt
let pattern: 1, 2, 3
for pattern: a at 0
for pattern: b at 1
function params: (3, 5)
if let pattern: green
while let pattern: 3
while let pattern: 2
while let pattern: 1
match pattern: got five
```

## 提示

- 这一节重点不是某一种单独语法，而是“模式到底能出现在哪些地方”
- `while let` 很适合“只要还能匹配成功就继续循环”的场景
