# 练习：迭代器

来源章节：`059-3-1-2-使用迭代器处理元素序列.md`

## 练习目标

练习以下概念：

- `iter`
- `into_iter`
- `filter`
- `map`
- `sum`
- 自定义 `Iterator`

## 要求

1. 用 `iter/filter/map/sum` 对偶数翻倍后求和
2. 用 `into_iter/filter/collect` 过滤指定尺码的鞋
3. 实现 `Counter` 迭代器，使它依次返回 `1..=5`
4. 用 `zip + skip + map + filter + sum` 复现书中的链式迭代器例子

## 目标输出

```txt
sum of doubled evens: 24
filtered shoes: 2
counter zip sum: 18
```

## 提示

- 迭代器是惰性的，只有被消费时才会真正执行
- `iter()` 产生引用，`into_iter()` 会获取集合所有权
- 自定义迭代器的关键是实现 `next`
- 建议同时运行 `cargo run` 和 `cargo test` 来理解每个部分
