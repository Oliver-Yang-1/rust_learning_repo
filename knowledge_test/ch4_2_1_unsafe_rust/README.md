# 练习：不安全 Rust

来源章节：`090-4-2-1-不安全的-Rust.md`

## 练习目标

练习以下概念：

- `unsafe` 块
- 裸指针
- 不安全函数
- 用安全抽象封装不安全代码

## 要求

1. 实现一个接收裸指针的不安全函数
2. 在测试里用 `unsafe` 块调用它
3. 实现一个安全版 `split_at_mut_safe`
4. 用测试验证切片被正确拆分

## 运行方式

```bash
cargo test
```

## 目标行为

- `raw_pointer_can_be_dereferenced_in_unsafe_block` 测试通过
- `split_at_mut_safe_splits_without_overlap` 测试通过

## 提示

- 重点不是“多写 unsafe”，而是“把 unsafe 缩到尽量小并封装起来”
- `unsafe` 不会关闭借用检查器，它只是允许你做编译器无法替你证明安全的事
