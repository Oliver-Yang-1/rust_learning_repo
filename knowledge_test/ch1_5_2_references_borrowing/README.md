# 练习：引用与借用

来源章节：`016-1-5-2-引用与借用.md`

## 练习目标

练习以下概念：

- 不可变引用
- 可变引用
- 借用读取数据而不获取所有权
- 同时多个不可变引用
- 用作用域缩短可变借用的生命周期

## 要求

1. 写一个函数 `calculate_length(text: &String) -> usize`
2. 写一个函数 `append_world(text: &mut String)`，把 `", world"` 追加到字符串后面
3. 先借用字符串求长度，再打印长度
4. 用一个额外的作用域创建可变引用并完成修改
5. 修改结束后，再创建两个不可变引用并同时打印

## 目标输出

```txt
length before change: 5
after change: hello, world
shared borrows: hello, world | hello, world
```

## 提示

- `&String` 是不可变借用，只能读
- `&mut String` 是可变借用，可以改内容
- 同一时刻只能有一个可变引用，或多个不可变引用
- 可以用 `{}` 人为缩短借用的作用域
