# 练习：if let 简洁控制流

来源章节：`025-1-7-3-if-let-简洁控制流.md`

## 练习目标

练习以下概念：

- 用 `if let` 处理只关心一种模式的情况
- `if let ... else`
- 与 `match` 的适用场景对比
- 匹配带数据的枚举成员

## 要求

1. 定义 `Coin` 枚举，包含 `Penny`、`Nickel`、`Dime`、`Quarter(String)`
2. 当硬币是 `Quarter(state)` 时打印州名
3. 否则统计到 `other_count`
4. 再定义一个 `Option<u8>`，只在它是 `Some(3)` 时打印 `three`
5. 最后打印 `other_count`

## 目标输出

```txt
state quarter from Alaska
three
other count: 1
```

## 提示

- `if let Coin::Quarter(state) = coin` 可以直接取出 `state`
- `else` 分支适合处理“其他所有情况”
- 只关心一个模式时，`if let` 比 `match` 更简洁
