# 练习：定义枚举

来源章节：`023-1-7-1-定义枚举.md`

## 练习目标

练习以下概念：

- 定义简单枚举
- 定义带数据的枚举
- 不同成员可携带不同类型的数据
- 在枚举上定义方法
- 使用 `Option<T>`

## 要求

1. 定义 `IpAddr` 枚举，包含 `V4(u8, u8, u8, u8)` 和 `V6(String)`
2. 定义 `Message` 枚举，包含 `Quit`、`Write(String)`、`ChangeColor(i32, i32, i32)`
3. 为 `Message` 实现方法 `call(&self) -> &'static str`
4. 创建一个 IPv4 地址和一个 IPv6 地址并打印
5. 创建一个 `Option<&str>`，用 `match` 打印是否有值

## 目标输出

```txt
home ipv4: 127.0.0.1
loopback ipv6: ::1
message call: message handled
optional text: rust
```

## 提示

- 枚举成员可以像函数一样传参
- `impl Message` 可以给枚举加方法
- `Some("rust")` 和 `None` 都属于 `Option<&str>`
