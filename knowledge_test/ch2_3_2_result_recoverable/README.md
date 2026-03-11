# 练习：Result 与可恢复错误

来源章节：`039-2-3-2-Result-与可恢复的错误.md`

## 练习目标

练习以下概念：

- `Result<T, E>`
- `match` 处理 `Ok` / `Err`
- `?` 运算符传播错误
- 在 `main` 中返回 `Result`

## 要求

1. 写一个函数 `parse_port(text: &str) -> Result<u16, std::num::ParseIntError>`
2. 用 `?` 将字符串解析成 `u16`
3. 写一个函数 `describe_port(text: &str) -> Result<String, std::num::ParseIntError>`
4. 在 `main` 中返回 `Result<(), Box<dyn std::error::Error>>`
5. 用合法输入 `"8080"` 调用并打印结果

## 目标输出

```txt
parsed port: 8080
description: server port is 8080
```

## 提示

- `text.parse::<u16>()` 会返回 `Result`
- `?` 只能用在返回 `Result` 或兼容类型的函数里
- `main` 也可以返回 `Result`
