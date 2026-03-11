# 练习：错误处理总览

来源章节：`037-2-3-错误处理.md`

## 练习目标

练习以下概念：

- 区分可恢复错误和不可恢复错误
- `Result<T, E>`
- `panic!`
- 通过 `match` 处理成功与失败

## 要求

1. 写一个函数 `safe_divide(a: i32, b: i32) -> Result<i32, String>`
2. 当 `b == 0` 时返回 `Err`
3. 否则返回除法结果 `Ok`
4. 再写一个函数 `require_positive(n: i32)`，当 `n <= 0` 时 `panic!`
5. 在 `main` 中只传入有效值，并打印成功结果

## 目标输出

```txt
divide ok: 5
positive check passed
```

## 提示

- 可恢复问题优先考虑返回 `Result`
- 真正违反前置条件或不变量时再考虑 `panic!`
- 题目默认使用合法输入，保证程序能正常运行
