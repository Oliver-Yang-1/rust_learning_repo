# 练习：采用测试驱动开发完善库的功能

来源章节：`053-2-6-4-采用测试驱动开发完善库的功能.md`

## 练习目标

练习以下概念：

- 为库功能写测试
- `search<'a>`
- 返回匹配行的 `Vec<&'a str>`
- 在 `run` 中调用 `search`

## 要求

1. 阅读 `src/lib.rs` 中的 `search` 实现和测试
2. 运行 `cargo test`
3. 再运行 `cargo run frog poem.txt`
4. 确认程序只打印匹配行

## 推荐命令

```bash
cargo test
cargo run frog poem.txt
```

## 目标结果

```txt
How public, like a frog
```

## 提示

- `search` 返回的是对 `contents` 中各行的借用
- 这就是为什么函数签名里需要生命周期 `'a`
- 测试先行特别适合这种“纯逻辑函数”
