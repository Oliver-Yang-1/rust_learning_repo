# 练习：trait 对象

来源章节：`082-3-5-2-为使用不同类型的值而设计的-trait-对象.md`

## 练习目标

练习以下概念：

- `trait`
- `Box<dyn Trait>`
- trait 对象
- 动态分发

## 要求

1. 定义 `Draw` trait
2. 定义 `Screen`，让它保存 `Vec<Box<dyn Draw>>`
3. 实现两个不同组件类型：`Button` 和 `SelectBox`
4. 让 `Screen::run` 统一调用它们的 `draw`

## 目标输出

```txt
SelectBox(75, 10, [Yes | Maybe | No])
Button(50, 10, OK)
```

## 提示

- 这里的关键是“同一个集合里装不同类型，但它们共享同一个行为接口”
- 如果用泛型 `Vec<T>`，同一个 `Screen` 实例里就只能放一种具体类型
