# 练习：实用开发工具

来源附录：

- 附录 D：实用开发工具

## 练习目标

练习这些常见工具：

- `cargo fmt`
- `cargo fix`
- `cargo clippy`

## 项目说明

当前代码已经是“正确版本”，其中 `circle_area` 使用了标准库里的 `PI` 常量，便于你对比理解为什么 `clippy` 会反对手写近似值。

## 建议练法

1. 先运行程序，确认输出
2. 执行 `cargo fmt -- --check`
3. 执行 `cargo clippy`
4. 自己把 `PI` 临时改成 `3.1415`
5. 再运行 `cargo clippy`，观察 `approx_constant` 提示
6. 把循环变量改成未使用名字，比如 `for radius in radii { ... }` 改成 `for i in 0..radii.len() { ... }`
7. 运行 `cargo fix --allow-dirty`，观察编译器建议如何自动应用

## 运行方式

```txt
cargo run
cargo test
```

## 目标输出

```txt
radius=1, area=3.14
radius=2.5, area=19.63
radius=4, area=50.27
```

## 提示

- `fmt` 关注格式
- `fix` 关注编译器可自动采纳的修复建议
- `clippy` 关注风格、正确性和经验性改进建议
