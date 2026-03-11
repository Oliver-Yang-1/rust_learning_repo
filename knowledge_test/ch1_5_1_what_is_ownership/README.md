# 练习：什么是所有权

来源章节：`015-1-5-1-什么是所有权.md`

## 练习目标

练习以下概念：

- 作用域结束时自动释放
- `String` 与堆数据
- 移动（move）
- 克隆（clone）
- `Copy` 类型

## 要求

1. 写一个函数 `build_message() -> String`，返回 `"hello"`
2. 在一个内部作用域中调用它并打印结果
3. 定义 `owner1 = String::from("Ferris")`，把它移动给 `owner2`
4. 基于 `owner2` 调用 `clone()` 得到 `owner3`
5. 再定义两个整数，展示 `Copy` 行为

## 目标输出

```txt
message in scope: hello
moved owner: Ferris
cloned owners: Ferris / Ferris
copy values: 42 and 42
```

## 提示

- 作用域结束时，拥有者会被丢弃
- `String` 赋值默认是移动，不是深拷贝
- `clone()` 才会复制堆上的内容
- `i32` 赋值是复制值本身
