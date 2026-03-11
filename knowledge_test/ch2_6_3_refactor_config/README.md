# 练习：重构以改进模块化与错误处理

来源章节：`052-2-6-3-重构以改进模块化与错误处理.md`

## 练习目标

练习以下概念：

- 将逻辑从 `main.rs` 提取到 `lib.rs`
- `Config` 结构体
- `Config::new`
- `run` 函数
- 在 `main` 中集中处理错误

## 要求

1. 阅读 `src/main.rs` 和 `src/lib.rs`
2. 理解为什么 `main` 只负责参数收集和错误出口
3. 运行程序并确认能打印匹配行

## 运行方式

```bash
cargo run frog poem.txt
```

## 目标输出

```txt
How public, like a frog
```

## 提示

- `main.rs` 越薄越容易验证
- 业务逻辑放在 `lib.rs` 更方便测试
- `Config::new` 返回 `Result` 比直接 panic 更友好
