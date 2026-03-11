# 练习：引用循环与 `Weak<T>`

来源章节：`074-3-3-6-引用循环与内存泄漏是安全的.md`

## 练习目标

练习以下概念：

- 引用循环
- `Weak<T>`
- `Rc::downgrade`
- `upgrade`
- `strong_count` 和 `weak_count`

## 要求

1. 创建一个 `leaf` 节点
2. 创建一个 `branch` 节点，并让它拥有 `leaf`
3. 让 `leaf.parent` 保存 `Weak<Node>`，而不是 `Rc<Node>`
4. 在 `branch` 作用域结束前后分别观察父节点和引用计数变化

## 输出格式

输出格式应类似：

```txt
leaf parent at start: None
leaf strong = 1, weak = 0
branch strong = 1, weak = 1
leaf parent in scope: Some(5)
branch children count: 1
leaf parent after scope: None
leaf strong = 1, weak = 0
```

## 提示

- `Weak<T>` 不拥有值，所以不会增加强引用计数
- `upgrade()` 会返回 `Option<Rc<T>>`
- 这题的重点是理解如何避免 `Rc<T>` 强引用环
