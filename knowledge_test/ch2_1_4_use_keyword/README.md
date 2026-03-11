# 练习：使用 use 关键字将名称引入作用域

来源章节：`031-2-1-4-使用-use-关键字将名称引入作用域.md`

## 练习目标

练习以下概念：

- `use` 导入模块
- 导入标准库类型
- 嵌套路径
- `as` 起别名

## 要求

1. 定义 `front_of_house::hosting::add_to_waitlist`
2. 用 `use crate::front_of_house::hosting;` 简化调用
3. 导入 `HashMap`
4. 用 `as` 给 `std::io::Result` 起别名
5. 在 `main` 中打印等待列表次数，并创建一个 `HashMap`

## 目标输出

```txt
added to waitlist
added to waitlist
table 1: Alice
alias ready: true
```

## 提示

- 对函数通常导入父模块更符合习惯
- 对结构体一般直接导入完整路径
- `use std::io::Result as IoResult;` 可以避免重名
