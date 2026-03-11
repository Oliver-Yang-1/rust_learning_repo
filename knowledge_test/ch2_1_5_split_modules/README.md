# 练习：将模块分割进不同文件

来源章节：`032-2-1-5-将模块分割进不同文件.md`

## 练习目标

练习以下概念：

- `mod xxx;` 从其他文件加载模块
- 在 `src/front_of_house.rs` 中声明子模块
- 在 `src/front_of_house/hosting.rs` 中定义函数
- 主文件调用跨文件模块中的函数

## 要求

1. 在 `src/main.rs` 中写 `mod front_of_house;`
2. 新建 `src/front_of_house.rs`
3. 在其中声明 `pub mod hosting;`
4. 新建 `src/front_of_house/hosting.rs`
5. 在 `hosting.rs` 中实现 `add_to_waitlist`
6. 在 `main` 中调用两次

## 目标输出

```txt
waitlist guest: Alice
waitlist guest: Bob
service ready
```

## 提示

- `mod front_of_house;` 后面要用分号，不是代码块
- Rust 会按模块名查找同名文件
- 继续拆分子模块时，要创建同名目录
