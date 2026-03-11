# 练习：模块系统总览

来源章节：`027-2-1-使用包、Crate-和模块管理不断增长的项目.md`

## 练习目标

练习以下概念：

- 用模块组织代码
- 用 `pub` 暴露公共接口
- 用 `use` 将模块引入作用域
- 区分“内部实现”和“对外接口”

## 要求

1. 定义模块 `front_of_house::hosting`
2. 定义模块 `back_of_house`
3. 让 `hosting::add_to_waitlist` 成为可调用的公共函数
4. 用 `use` 引入 `hosting`
5. 在 `main` 中打印欢迎信息，并调用两次 `add_to_waitlist`

## 目标输出

```txt
theme: packages crates modules
added to waitlist
added to waitlist
kitchen ready
```

## 提示

- 模块默认是私有的
- `pub` 只会暴露你明确标记的项
- `use crate::front_of_house::hosting;` 可以简化调用路径
