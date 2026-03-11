# 练习：定义模块、作用域与私有性

来源章节：`029-2-1-2-定义模块来控制作用域与私有性.md`

## 练习目标

练习以下概念：

- 嵌套模块
- `pub mod`
- `pub fn`
- 公有结构体与私有字段
- 公有枚举

## 要求

1. 定义 `front_of_house::hosting` 模块，并暴露 `add_to_waitlist`
2. 定义 `back_of_house::Breakfast` 结构体
3. 让 `Breakfast` 的 `toast` 字段公有，另一个字段私有
4. 为 `Breakfast` 提供公共关联函数 `summer`
5. 定义公有枚举 `Appetizer`
6. 在 `main` 中修改 `toast`，并打印早餐与开胃菜

## 目标输出

```txt
added to waitlist
toast: Wheat
appetizer: Soup
```

## 提示

- `pub struct` 不等于所有字段都公有
- 如果结构体有私有字段，通常要提供公共构造函数
- `pub enum` 的成员默认就是公有的
