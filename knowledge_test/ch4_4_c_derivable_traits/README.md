# 练习：可派生的 trait

来源附录：

- 附录 C：可派生的 trait

## 练习目标

通过一个 `Task` 类型和一个 `Point` 类型，练习这些可派生 trait：

- `Debug`
- `PartialEq` / `Eq`
- `PartialOrd` / `Ord`
- `Clone`
- `Copy`
- `Hash`
- `Default`

## 要求

1. 为 `Task` 一次性派生多个 trait
2. 用 `assert_eq!` 依赖 `Debug + PartialEq`
3. 用 `BTreeSet` 依赖 `Ord`
4. 用 `HashMap` 键依赖 `Hash + Eq`
5. 用 `Default::default()` 生成默认值
6. 用 `Point` 展示 `Copy`

## 运行方式

```txt
cargo test
```

## 预期结果

所有测试通过。

## 提示

- `Copy` 往往只适合“小且按位复制即可”的类型
- `Task` 里有 `String`，所以它不能实现 `Copy`，但可以实现 `Clone`
