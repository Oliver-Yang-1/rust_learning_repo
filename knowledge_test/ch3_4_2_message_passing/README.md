# 练习：消息传递

来源章节：`077-3-4-2-消息传递.md`

## 练习目标

练习以下概念：

- `mpsc::channel`
- `send`
- `recv` / 遍历接收端
- 多生产者单消费者

## 要求

1. 创建一个 channel
2. 克隆发送端，构造两个生产者
3. 让两个线程分别发送字符串消息
4. 在主线程中接收全部消息
5. 为了便于校验输出，将接收到的消息排序后打印

## 目标输出

```txt
received count: 4
received messages: alpha, beta, delta, gamma
```

## 提示

- `mpsc` 是 multiple producer, single consumer 的缩写
- `send` 会取得消息所有权
- 当所有发送端都被丢弃后，`for message in rx` 才会结束
