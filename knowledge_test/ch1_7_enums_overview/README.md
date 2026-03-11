# 练习：枚举与模式匹配总览

来源章节：`022-1-7-枚举与模式匹配.md`

## 练习目标

练习以下概念：

- 定义枚举
- 为枚举成员附加数据
- `Option<T>`
- `match` 模式匹配
- 从匹配中取出成员内部的值

## 要求

1. 定义枚举 `TaskStatus`，包含 `Todo`、`Doing`、`Done`
2. 定义枚举 `Message`，包含 `Quit`、`Write(String)`、`Move { x: i32, y: i32 }`
3. 写函数 `describe_status(status: TaskStatus) -> &'static str`
4. 写函数 `plus_one(x: Option<i32>) -> Option<i32>`
5. 在 `main` 中分别打印状态说明、消息内容、以及 `Option` 处理结果

## 目标输出

```txt
status: task is in progress
message: note = study enums
move to: (3, 5)
plus one some: Some(6)
plus one none: None
```

## 提示

- `match` 既可以匹配成员，也可以绑定成员中的数据
- `Option<i32>` 需要同时处理 `Some` 和 `None`
- 结构体式枚举成员可以写成 `Move { x, y }`
