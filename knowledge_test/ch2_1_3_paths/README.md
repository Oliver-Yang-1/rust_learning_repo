# 练习：路径用于引用模块树中的项

来源章节：`030-2-1-3-路径用于引用模块树中的项.md`

## 练习目标

练习以下概念：

- 绝对路径
- 相对路径
- `super`
- 从父模块访问公有项

## 要求

1. 定义 `front_of_house::hosting::add_to_waitlist`
2. 在 `main` 中分别用绝对路径和相对路径调用它
3. 定义 `serve_order`
4. 在 `back_of_house` 模块里用 `super::serve_order()` 调用父模块函数

## 目标输出

```txt
added by absolute path
added by relative path
order served
```

## 提示

- 绝对路径通常以 `crate::` 开头
- 相对路径从当前模块开始写
- `super` 表示父模块
