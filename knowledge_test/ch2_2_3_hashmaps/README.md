# 练习：哈希 map

来源章节：`036-2-2-3-哈希-map.md`

## 练习目标

练习以下概念：

- 创建 `HashMap`
- `insert`
- `get`
- `entry(...).or_insert(...)`
- 根据旧值更新值

## 要求

1. 创建一个 `HashMap<String, i32>`
2. 插入 `Blue -> 10` 和 `Yellow -> 50`
3. 读取 `Blue` 的分数并打印
4. 用相同键再次插入 `Blue -> 25`，展示覆盖旧值
5. 用 `entry(...).or_insert(...)` 给 `Red` 插入默认值 `30`
6. 统计 `"hello world hello rust"` 中每个单词出现的次数，并打印 `hello` 的计数

## 目标输出

```txt
blue score: 10
blue replaced: 25
red score: 30
hello count: 2
```

## 提示

- `get` 返回 `Option<&V>`
- 相同键再次 `insert` 会覆盖旧值
- `or_insert` 会返回该键对应值的可变引用
