# 练习：字符串

来源章节：`035-2-2-2-字符串.md`

## 练习目标

练习以下概念：

- 创建 `String`
- `push_str` 和 `push`
- `format!`
- 字符串 slice
- `chars()` 遍历 UTF-8 文本

## 要求

1. 创建 `String::from("Rust")`
2. 用 `push_str` 和 `push` 把它变成 `Rust!`
3. 用 `format!` 拼接出 `Rust! language`
4. 从 `"hello world"` 中切出前半段 `hello`
5. 用 `chars()` 统计 `"你好"` 的字符数

## 目标输出

```txt
message: Rust!
combined: Rust! language
slice: hello
char count: 2
```

## 提示

- `push_str` 接收 `&str`
- `push` 接收单个 `char`
- 字符串不能像数组那样直接按索引取字符
- 对 UTF-8 文本更稳妥的方式是用 `chars()`
