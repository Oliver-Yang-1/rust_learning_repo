# 练习：生命周期与引用有效性

来源章节：`044-2-4-3-生命周期与引用有效性.md`

## 练习目标

练习以下概念：

- 函数中的生命周期参数
- 返回引用时描述输入和输出的关系
- 含引用字段的结构体
- 生命周期省略与显式标注的区别

## 要求

1. 写函数 `longest<'a>(x: &'a str, y: &'a str) -> &'a str`
2. 定义结构体 `ImportantExcerpt<'a>`，字段 `part: &'a str`
3. 为它实现方法 `level(&self) -> i32`
4. 再实现方法 `announce_and_return_part(&self, announcement: &str) -> &str`
5. 在 `main` 中创建一个字符串、切出第一句、构造 `ImportantExcerpt` 并打印结果

## 目标输出

```txt
longest: abcd
level: 3
excerpt: Call me Ishmael
```

## 提示

- 生命周期注解描述“谁至少活多久”
- `announce_and_return_part` 的返回值会因 `&self` 受益于生命周期省略规则
- 结构体里只要有引用字段，就通常需要显式生命周期参数
