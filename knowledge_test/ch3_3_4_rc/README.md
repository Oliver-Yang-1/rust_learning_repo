# 练习：`Rc<T>`

来源章节：`072-3-3-4-Rc-引用计数智能指针.md`

## 练习目标

练习以下概念：

- `Rc<T>`
- `Rc::clone`
- `Rc::strong_count`
- 共享所有权

## 要求

1. 创建一个共享尾部链表 `shared_tail`
2. 用两个不同列表共享它
3. 在不同阶段打印引用计数
4. 观察内部作用域结束后计数如何减少

## 目标输出

```txt
count after creating shared_tail: 1
count after creating list_b: 2
count after creating list_c: 3
count after list_c goes out of scope: 2
```

## 提示

- `Rc::clone` 不会深拷贝整份数据，只会增加引用计数
- `Rc<T>` 适合单线程的多所有者场景
