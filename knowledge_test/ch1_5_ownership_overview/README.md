# 练习：所有权总览

来源章节：`014-1-5-认识所有权.md`

## 练习目标

练习以下概念：

- `String` 的移动（move）
- `clone` 深拷贝
- 借用读取数据
- `Copy` 类型复制
- 字符串 slice 的基本使用

## 要求

1. 创建一个 `String` 变量 `topic = "Rust ownership"`
2. 把它移动给 `moved_topic`，并打印移动后的值
3. 使用 `clone` 创建 `backup_topic`
4. 写一个函数 `borrow_length(text: &String) -> usize`
5. 定义两个整数，展示 `Copy` 行为
6. 再写一个函数 `first_word(text: &str) -> &str`，取出 `"ownership rules matter"` 的第一个单词

## 目标输出

```txt
moved topic: Rust ownership
backup topic: Rust ownership
borrowed length: 14
copied numbers: 7 and 7
first word: ownership
```

## 提示

- `let moved_topic = topic;` 会发生移动
- `clone()` 会复制堆上的数据
- `i32` 是 `Copy` 类型
- `&str` 比 `&String` 更通用
