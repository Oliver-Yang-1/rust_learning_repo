# 练习：优雅停机与清理

来源章节：`098-4-3-3-优雅停机与清理.md`

## 练习目标

练习以下概念：

- `Drop` 在线程池中的作用
- `Message::NewJob` / `Message::Terminate`
- `Option<JoinHandle<()>>`
- 在清理时 `join` 所有线程

## 要求

1. 在线程池中引入 `Message` 枚举
2. 让 worker 同时处理新任务和终止消息
3. 为 `ThreadPool` 实现 `Drop`
4. 在 `drop` 中先发送终止消息，再 `join` 所有线程
5. 用测试验证线程池销毁前会等任务执行完

## 运行方式

```bash
cargo test
```

## 目标行为

- `drop_waits_for_running_jobs_to_finish` 测试通过
- `all_jobs_complete_before_shutdown_returns` 测试通过

## 提示

- 这一题的重点不是“停机”，而是“有序停机”
- 如果不 `join` worker，主线程结束时很容易把仍在运行的任务直接掐掉
