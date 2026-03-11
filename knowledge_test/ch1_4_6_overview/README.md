# 综合练习：第 1.4 节回顾

来源章节：`008-1-4-常见编程概念.md`

## 练习目标

写一个小程序，综合使用以下内容：

- 变量与隐藏
- 至少 2 种数据类型
- 至少 1 个带返回值的函数
- 至少 1 个 `if` 表达式
- 至少 1 个循环

## 要求

1. 定义一个课程名常量 `COURSE_NAME`
2. 定义学生名 `name` 和年龄 `age`
3. 使用隐藏把年龄变成“明年年龄”
4. 写一个函数 `score_to_level(score: i32) -> char`
5. 用 `if` 表达式生成 `status`
6. 用循环打印倒计时

## 目标输出

```txt
course: Rust Common Concepts
name: Ferris
age next year: 19
score level: A
status: pass
countdown:
3!
2!
1!
GO!
```

## 提示

- `score >= 90` 可以返回 `'A'`
- `if` 是表达式，可以直接赋值给变量
- 倒计时可以用 `while` 或 `for`
