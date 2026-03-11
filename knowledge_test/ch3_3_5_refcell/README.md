# 练习：`RefCell<T>` 与内部可变性

来源章节：`073-3-3-5-RefCell-与内部可变性模式.md`

## 练习目标

练习以下概念：

- `RefCell<T>`
- 运行时借用检查
- 内部可变性
- `Rc<RefCell<T>>`
- mock 对象测试

## 要求

1. 阅读 `LimitTracker` 和 `Messenger` 设计
2. 用 `RefCell<Vec<String>>` 实现 `MockMessenger`
3. 运行 `cargo test`，确认超过 75% 时会记录一条消息
4. 观察 `Rc<RefCell<i32>>` 如何让多个所有者共享并修改同一个值

## 运行方式

```bash
cargo test
```

## 目标行为

- `sends_warning_message_when_over_75_percent` 测试通过
- `rc_refcell_allows_shared_mutation` 测试通过

## 提示

- `borrow()` 对应不可变借用
- `borrow_mut()` 对应可变借用
- 如果同时存在多个可变借用，`RefCell<T>` 会在运行时 panic
