# 练习：`Send` 与 `Sync`

来源章节：`079-3-4-4-可扩展的并发-Sync-与-Send.md`

## 练习目标

练习以下概念：

- `Send`
- `Sync`
- 编译期约束
- 为什么 `Arc<T>` 适合并发，而 `Rc<T>` 不适合

## 要求

1. 编写 `assert_send<T: Send>()`
2. 编写 `assert_sync<T: Sync>()`
3. 验证 `Arc<Mutex<i32>>` 同时满足 `Send` 和 `Sync`
4. 把共享值移到线程里修改，再回到主线程读取结果

## 目标输出

```txt
send + sync check passed
shared value after thread: 42
```

## 提示

- `Send` 表示所有权可以在线程间移动
- `Sync` 表示引用可以安全地在线程间共享
- 这一章很多知识点体现在“能否编译通过”上
