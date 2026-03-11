# 练习：Slice 类型

来源章节：`017-1-5-3-Slices.md`

## 练习目标

练习以下概念：

- `&str` 字符串 slice
- 从 `String` 中取出一段切片
- 让函数参数使用 `&str`
- 字符串字面值本身就是 `&str`
- 数组 slice

## 要求

1. 写一个函数 `first_word(text: &str) -> &str`
2. 让它能处理 `String` 和字符串字面值两种输入
3. 从 `message = "hello world"` 中取出第一个单词
4. 打印整个字符串的 slice
5. 对数组 `[10, 20, 30, 40, 50]` 取出一个中间 slice 并打印

## 目标输出

```txt
first word from String: hello
first word from literal: rust
whole slice: hello world
number slice: [20, 30, 40]
```

## 提示

- `&String` 可以转成 `&str`
- 找到空格后可以返回 `&text[..index]`
- 如果没有空格，就返回整个 `text`
- 数组切片的类型类似 `&[i32]`
