# 练习：包和 crate

来源章节：`028-2-1-1-包和-crate.md`

## 练习目标

练习以下概念：

- 一个包可以同时包含二进制 crate 和库 crate
- `src/main.rs` 是二进制 crate 根
- `src/lib.rs` 是库 crate 根
- 二进制 crate 可以调用同包内库 crate 的公共函数

## 要求

1. 保留 `src/main.rs` 作为二进制入口
2. 新建 `src/lib.rs`
3. 在 `src/lib.rs` 中定义两个公共函数：
   `library_name() -> &'static str`
   `shared_message() -> &'static str`
4. 在 `main` 中调用这两个函数并打印结果

## 目标输出

```txt
binary crate: running
library crate: shared utilities
message: package can contain multiple crates
```

## 提示

- 在同一个包里，二进制 crate 可以通过包名访问库 crate
- 这里的包名就是 `ch2_1_1_packages_crates`
- 可以写 `use ch2_1_1_packages_crates::shared_message;`
