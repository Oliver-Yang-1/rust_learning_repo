# 练习：定义并实例化结构体

来源章节：`019-1-6-1-定义并实例化结构体.md`

## 练习目标

练习以下概念：

- 定义带命名字段的结构体
- 创建结构体实例
- 修改可变结构体字段
- 字段初始化简写
- 结构体更新语法

## 要求

1. 定义结构体 `User`，包含 `username`、`email`、`active`、`sign_in_count`
2. 写一个函数 `build_user(email: String, username: String) -> User`
3. 创建可变实例 `user1`，把登录次数改成 `2`
4. 修改 `user1` 的邮箱
5. 基于 `user1` 创建 `user2`

## 目标输出

```txt
user1: ferris / ferris_new@rust.dev / true / 2
user2: crab / crab@rust.dev / true / 2
```

## 提示

- 整个结构体实例要声明为 `mut`，字段才能改
- `build_user` 中可以直接写 `email, username`
- 使用 `..user1` 时，未显式赋值的字段会从 `user1` 复制或移动过来
