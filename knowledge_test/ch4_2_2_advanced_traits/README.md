# 练习：高级 trait

来源章节：`091-4-2-2-高级-trait.md`

## 练习目标

练习以下概念：

- 运算符重载
- 完全限定语法
- 父 trait
- newtype 模式

## 要求

1. 为 `Point` 实现 `Add`
2. 实现 `Pilot`、`Wizard` 和 `Human` 的同名方法，并用完全限定语法调用
3. 用 `<Dog as Animal>::baby_name()` 调用 trait 关联函数
4. 实现 `OutlinePrint: Display`
5. 用 `Wrapper(Vec<String>)` 演示 newtype + `Display`

## 运行方式

```bash
cargo test
```

## 目标行为

- `Point + Point` 能正确相加
- 不同 `fly` 实现能被正确区分
- `Dog::baby_name()` 和 `<Dog as Animal>::baby_name()` 返回不同结果
- `Wrapper` 能被格式化输出

## 提示

- 这一章不只是在讲 trait，本质上是在讲“当 trait 复杂起来时如何保持调用清晰”
- 完全限定语法的核心形式是 `<Type as Trait>::function(...)`
