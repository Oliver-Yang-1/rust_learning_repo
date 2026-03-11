# 练习：方法语法

来源章节：`021-1-6-3-方法语法.md`

## 练习目标

练习以下概念：

- 在 `impl` 块中定义方法
- 使用 `&self`
- 为方法传入额外参数
- 定义关联函数
- 使用 `Type::function()` 调用关联函数

## 要求

1. 定义 `Rectangle` 结构体
2. 在 `impl Rectangle` 中实现方法 `area(&self) -> u32`
3. 实现方法 `can_hold(&self, other: &Rectangle) -> bool`
4. 实现关联函数 `square(size: u32) -> Rectangle`
5. 在 `main` 中创建 `rect1`、`rect2`、`rect3`，并验证面积与包含关系

## 目标输出

```txt
rect1 area: 1500
rect1 can hold rect2: true
rect1 can hold rect3: false
square area: 9
```

## 提示

- `&self` 表示不可变借用当前实例
- `can_hold` 需要比较两个长方形的宽和高
- 关联函数没有 `self` 参数，常用来创建实例
