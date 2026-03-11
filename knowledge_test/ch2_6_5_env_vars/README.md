# 练习：处理环境变量

来源章节：`054-2-6-5-处理环境变量.md`

## 练习目标

练习以下概念：

- 环境变量控制程序行为
- `CASE_INSENSITIVE`
- 大小写敏感与不敏感搜索
- 为新功能补测试

## 要求

1. 阅读 `src/lib.rs` 中的两个搜索函数
2. 运行 `cargo test`
3. 先正常运行一次大小写敏感搜索
4. 再设置环境变量运行大小写不敏感搜索

## 推荐命令

```bash
cargo test
cargo run to poem.txt
CASE_INSENSITIVE=1 cargo run to poem.txt
```

## 目标结果

- 默认搜索时，不会匹配大写开头的 `To`
- 设置 `CASE_INSENSITIVE=1` 后，会额外匹配：

```txt
To tell your name the livelong day
To an admiring bog!
```

## 提示

- `env::var("CASE_INSENSITIVE").is_err()` 是书里常见写法
- 测试里可以分别验证大小写敏感和不敏感两个函数
