# 练习：match 控制流

来源章节：`024-1-7-2-match-控制流运算符.md`

## 练习目标

练习以下概念：

- 使用 `match` 匹配枚举成员
- 在分支里绑定成员携带的数据
- 匹配 `Option<T>`
- 穷尽匹配
- 通配符分支

## 要求

1. 定义 `Coin` 枚举，包含 `Penny`、`Nickel`、`Dime`、`Quarter(String)`
2. 写函数 `value_in_cents(coin: Coin) -> u8`
3. 当遇到 `Quarter(state)` 时，先打印州名，再返回 `25`
4. 写函数 `plus_one(x: Option<i32>) -> Option<i32>`
5. 再写一个 `describe_number(x: u8) -> &'static str`，只对 `1`、`3`、`5` 特判，其余用 `_`

## 目标输出

```txt
coin value: 1
state quarter from Alaska
coin value: 25
plus one: Some(11)
number: three
```

## 提示

- `match` 的每个分支都要覆盖一种可能
- `Quarter(state)` 里的 `state` 是绑定出来的新变量
- `_` 可以匹配剩余所有情况
