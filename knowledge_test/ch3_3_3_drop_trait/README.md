# 练习：`Drop` Trait

来源章节：`071-3-3-3-Drop-Trait-运行清理代码.md`

## 练习目标

练习以下概念：

- `Drop` trait
- 离开作用域时自动清理
- `std::mem::drop`

## 要求

1. 定义一个实现了 `Drop` 的结构体
2. 在 `drop` 中打印被释放的资源名字
3. 使用 `std::mem::drop` 提前释放一个值
4. 让另一个值在作用域结束时自动释放

## 输出格式

```txt
before early drop
dropping: early resource
after early drop
before scope end: normal resource
dropping: normal resource
```

## 提示

- 不能直接手动调用 `Drop::drop`
- 如果要提前释放值，应该调用 `std::mem::drop`
