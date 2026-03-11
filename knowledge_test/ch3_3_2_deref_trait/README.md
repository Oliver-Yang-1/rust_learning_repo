# 练习：`Deref` Trait

来源章节：`070-3-3-2-通过-Deref-trait-将智能指针当作常规引用处理.md`

## 练习目标

练习以下概念：

- 自定义智能指针 `MyBox<T>`
- `Deref` trait
- `*` 解引用
- deref coercion

## 要求

1. 定义 `MyBox<T>`
2. 为 `MyBox<T>` 实现 `Deref`
3. 使用 `*` 取出 `MyBox<i32>` 中的值
4. 把 `&MyBox<String>` 传给接收 `&str` 的函数

## 目标输出

```txt
deref number: 5
deref greeting: Hello, Rust!
```

## 提示

- `deref` 应该返回内部值的引用
- deref coercion 会在函数参数匹配时自动发生
