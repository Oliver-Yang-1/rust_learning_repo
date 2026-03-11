# 练习：trait 定义共享行为

来源章节：`043-2-4-2-trait-定义共享的行为.md`

## 练习目标

练习以下概念：

- 定义 trait
- 为类型实现 trait
- 默认实现
- trait 作为参数
- trait bound

## 要求

1. 定义 trait `Summary`
2. 让它包含一个必须实现的方法 `summarize_author`
3. 再给它一个默认方法 `summarize`
4. 定义结构体 `Tweet` 并实现 `Summary`
5. 写函数 `notify<T: Summary>(item: &T)`
6. 在 `main` 中打印 `summarize()` 和 `notify()` 的结果

## 目标输出

```txt
summary: (Read more from @ferris...)
breaking news: (Read more from @ferris...)
```

## 提示

- 默认方法可以调用同一个 trait 中的其他方法
- `notify<T: Summary>` 是 trait bound 写法
- `impl Trait` 也能表达类似含义，但这里先练 trait bound
