# 练习：结构体示例程序

来源章节：`020-1-6-2-一个使用结构体的示例程序.md`

## 练习目标

练习以下概念：

- 用结构体表示一组相关数据
- 通过借用把结构体传给函数
- 计算长方形面积
- 为结构体派生 `Debug`
- 使用 `{:?}` 打印结构体

## 要求

1. 定义 `Rectangle` 结构体，包含 `width` 和 `height`
2. 为它加上 `#[derive(Debug)]`
3. 写一个函数 `area(rect: &Rectangle) -> u32`
4. 创建两个长方形并分别打印调试信息
5. 打印它们的面积

## 目标输出

```txt
rect1: Rectangle { width: 30, height: 50 }
rect1 area: 1500
rect2: Rectangle { width: 80, height: 60 }
rect2 area: 4800
```

## 提示

- `area` 应该借用 `Rectangle`，而不是获取所有权
- `width * height` 就是面积
- `{:?}` 需要 `Debug` trait 支持
