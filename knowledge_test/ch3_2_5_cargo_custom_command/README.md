# 练习：Cargo 自定义扩展命令

来源章节：`067-3-2-5-Cargo-自定义扩展命令.md`

## 练习目标

练习以下概念：

- `cargo-xxx` 命名规则
- 自定义 Cargo 子命令
- `cargo install --path .`
- `cargo --list`

## 要求

1. 先运行 `cargo run`
2. 执行 `cargo install --path .`
3. 安装后运行 `cargo hello-study`
4. 可选：运行 `cargo --list`，观察是否出现 `hello-study`

## 运行方式

```bash
cargo run
cargo install --path .
cargo hello-study
```

## 目标输出

```txt
custom cargo command works
subcommand name: hello-study
```

## 提示

- 只要二进制名称是 `cargo-hello-study`，就可以通过 `cargo hello-study` 调用
- 这正是 Cargo 扩展命令的工作方式
