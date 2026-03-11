# 练习：使用 cargo install 安装二进制文件

来源章节：`066-3-2-4-使用-cargo-install-从-Crates.io-安装二进制文件.md`

## 练习目标

练习以下概念：

- 只有二进制 crate 才能被安装
- `cargo install --path .`
- 安装后的可执行文件名称
- `$HOME/.cargo/bin` 的作用

## 要求

1. 先运行 `cargo run`
2. 再运行 `cargo install --path .`
3. 安装完成后，直接运行 `hello-install`
4. 确认输出与 `cargo run` 时一致

## 运行方式

```bash
cargo run
cargo install --path .
hello-install
```

## 目标输出

```txt
hello from cargo install
binary name: hello-install
```

## 提示

- 这里用本地路径安装，便于练习，不依赖 crates.io
- 如果 `hello-install` 无法直接运行，先检查 `$HOME/.cargo/bin` 是否在 `$PATH` 中
