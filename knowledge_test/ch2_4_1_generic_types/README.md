# 练习：泛型数据类型

来源章节：`042-2-4-1-泛型数据类型.md`

## 练习目标

练习以下概念：

- 泛型函数
- 泛型结构体
- 多个泛型参数
- 泛型方法

## 要求

1. 写一个泛型函数 `first_item<T: Copy>(list: &[T]) -> T`
2. 定义结构体 `Point<T, U>`，包含 `x` 和 `y`
3. 为 `Point<T, U>` 实现方法 `mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>`
4. 在 `main` 中用 `i32`、`char` 演示 `first_item`
5. 在 `main` 中创建两个不同类型的 `Point` 并调用 `mixup`

## 目标输出

```txt
first number: 10
first char: r
mixed point: x = 5, y = z
```

## 提示

- `T: Copy` 可以让你直接返回切片中的值
- `mixup` 方法里的 `V` 和 `W` 只属于这个方法本身
- `Point<T, U>` 可以让两个字段拥有不同类型
