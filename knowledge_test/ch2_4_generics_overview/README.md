# 练习：泛型、trait 与生命周期总览

来源章节：`041-2-4-泛型、trait-与生命周期.md`

## 练习目标

练习以下概念：

- 泛型函数
- trait 约束
- 带默认实现的 trait
- 生命周期标注

## 要求

1. 写一个泛型函数 `largest<T: PartialOrd + Copy>(list: &[T]) -> T`
2. 定义 trait `Label`，包含默认方法 `label(&self) -> String`
3. 定义结构体 `Book` 并实现 `Label`
4. 写一个函数 `longer<'a>(x: &'a str, y: &'a str) -> &'a str`
5. 在 `main` 中分别演示这三部分

## 目标输出

```txt
largest number: 9
label: [item]
longer text: rustacean
```

## 提示

- `largest` 里的 `T` 需要能比较大小，也要能拷贝
- trait 可以提供默认实现
- 生命周期标注描述引用之间的关系，不会延长值本身的生命周期
