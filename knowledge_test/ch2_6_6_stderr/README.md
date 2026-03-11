# 练习：将错误信息输出到标准错误

来源章节：`055-2-6-6-将错误信息输出到标准错误而不是标准输出.md`

## 练习目标

练习以下概念：

- `println!` 与 `eprintln!`
- 标准输出和标准错误的区别
- 重定向输出时的行为差异

## 要求

1. 阅读 `src/main.rs` 中的错误处理逻辑
2. 确认错误信息使用 `eprintln!`
3. 运行一个成功示例
4. 再运行一个缺参数示例，并观察错误信息仍然打印在终端

## 推荐命令

```bash
cargo run to poem.txt
cargo run > output.txt
cargo run to poem.txt > output.txt
```

## 目标结果

- 缺参数时，错误信息显示在终端，而不是写进 `output.txt`
- 正常运行时，匹配结果会进入 `output.txt`

## 提示

- `println!` 写到标准输出
- `eprintln!` 写到标准错误
- 这对命令行工具很重要，因为用户常常会重定向标准输出
