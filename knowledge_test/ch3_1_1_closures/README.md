# 练习：闭包

来源章节：`058-3-1-1-闭包-可以捕获其环境的匿名函数.md`

## 练习目标

练习以下概念：

- 闭包语法
- 类型推断
- 捕获环境
- `move` 闭包
- 用结构体缓存闭包结果

## 要求

1. 定义一个闭包，把 `"pushups"` 变成 `"workout-pushups"`
2. 定义一个捕获外部变量 `limit = 10` 的闭包，并判断 `8` 是否不超过 `limit`
3. 定义一个 `move` 闭包，输出 `tags` 中的字符串
4. 实现一个简单的 `Cacher`，同一个参数重复调用时不要重复计算

## 目标输出

```txt
closure label: workout-pushups
captured limit: true
move closure tags: rust, closure, move
cached 10: 20
cached 10 again: 20
cached 20: 40
computed times: 2
```

## 提示

- 闭包可以像函数一样接收参数并返回值
- 闭包可以直接读取外部作用域的变量
- `move` 会把需要的数据所有权移动到闭包中
- 缓存示例可以帮助理解书里 `Cacher` 的设计动机
