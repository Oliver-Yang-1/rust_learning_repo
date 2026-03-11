# 练习：运行测试

来源章节：`047-2-5-2-运行测试.md`

## 练习目标

练习以下概念：

- `cargo test`
- 通过名字过滤测试
- `-- --nocapture`
- `-- --ignored`
- `-- --test-threads=1`

## 要求

1. 先运行全部测试
2. 再按名字只运行 `add_` 相关测试
3. 用 `-- --nocapture` 观察打印输出
4. 用 `-- --ignored` 运行被忽略的测试

## 推荐命令

```bash
cargo test
cargo test add_
cargo test print -- --nocapture
cargo test -- --ignored
cargo test -- --test-threads=1
```

## 目标结果

- 默认运行时，有 1 个测试会被标记为 `ignored`
- 使用 `print -- --nocapture` 时，可以看到测试里的 `println!` 输出
- 使用 `-- --ignored` 时，只运行被忽略的测试

## 提示

- 传给 `cargo test` 和传给测试二进制的参数要用 `--` 分隔
- 名字过滤是“包含匹配”，不是必须全名一致
- 只在测试通过时想看输出，就用 `-- --nocapture`
