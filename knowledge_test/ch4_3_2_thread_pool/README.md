# 练习：将单线程 Server 变为多线程 Server

来源章节：`097-4-3-2-将单线程-server-变为多线程-server.md`

## 练习目标

练习以下概念：

- `ThreadPool`
- `Job = Box<dyn FnOnce() + Send + 'static>`
- `mpsc`
- `Arc<Mutex<Receiver<_>>>`

## 要求

1. 实现一个基础版 `ThreadPool`
2. 在 `new` 中创建固定数量的 worker
3. 在 `execute` 中把任务送进通道
4. 让 worker 持续从接收端取任务执行
5. 用测试验证多个任务都能被执行

## 运行方式

```bash
cargo test
```

## 目标行为

- `new_panics_when_size_is_zero` 测试通过
- `execute_runs_multiple_jobs` 测试通过
- `pool_reports_worker_count` 测试通过

## 提示

- 这一题先关注“任务如何被线程池分发”，还不要求优雅停机
- `Job` 类型别名会让签名清晰很多
