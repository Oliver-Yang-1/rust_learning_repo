# 练习：高级函数与闭包

来源章节：`093-4-2-4-高级函数与闭包.md`

## 练习目标

练习以下概念：

- 函数指针 `fn`
- 把函数作为参数传递
- 在 `map` 中传函数名
- 返回闭包

## 要求

1. 实现 `do_twice(f: fn(i32) -> i32, arg: i32)`
2. 把 `add_one` 作为参数传给 `do_twice`
3. 用函数名而不是闭包完成一次 `map`
4. 返回一个 `Box<dyn Fn(i32) -> i32>`

## 目标输出

```txt
function pointer answer: 12
mapped strings: 1, 2, 3
returned closure answer: 6
```

## 提示

- 函数指针实现了闭包 trait，所以很多期望闭包的位置也能传普通函数
- 闭包本身没有统一的具体类型，所以返回闭包时通常需要 `Box<dyn Fn(...) -> ...>`
