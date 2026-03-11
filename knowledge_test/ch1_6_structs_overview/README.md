# 练习：结构体总览

来源章节：`018-1-6-使用结构体来组织相关联的数据.md`

## 练习目标

练习以下概念：

- 定义普通结构体
- 使用函数返回结构体实例
- 字段初始化简写
- 结构体更新语法
- 元组结构体

## 要求

1. 定义结构体 `Book`，包含 `title`、`author`、`pages`、`published`
2. 写一个函数 `build_book(title: String, author: String) -> Book`
3. 创建 `book1` 并打印标题和作者
4. 使用结构体更新语法基于 `book1` 创建 `book2`
5. 定义元组结构体 `Rgb(u8, u8, u8)` 并打印颜色值

## 目标输出

```txt
book1: Rust Notes by Ferris
book1 pages: 120
book2: Rust Notes Advanced by Ferris
book2 published: true
accent color: 30, 144, 255
```

## 提示

- `build_book` 里可以用字段初始化简写
- `..book1` 可以复用旧实例中未显式指定的字段
- 元组结构体可以用 `.0`、`.1`、`.2` 访问字段
