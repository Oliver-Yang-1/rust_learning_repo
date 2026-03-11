# 练习：线程

来源章节：`076-3-4-1-线程.md`

## 练习目标

练习以下概念：

- `thread::spawn`
- `JoinHandle`
- `join`
- `move` 闭包

## 要求

1. 在主线程中创建一个 `Vec<i32>`
2. 使用 `thread::spawn` 创建新线程
3. 用 `move` 把数据所有权转移进新线程
4. 在新线程中计算总和并返回字符串
5. 在主线程中调用 `join` 等待结果

## 目标输出

```txt
main thread: preparing data
spawned thread sum: 6
main thread: finished
```

## 提示

- 如果不使用 `move`，闭包通常会尝试借用外部值
- `join()` 会阻塞当前线程直到子线程结束
