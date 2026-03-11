# 练习：高级类型

来源章节：`092-4-2-3-高级类型.md`

## 练习目标

练习以下概念：

- newtype 模式
- 类型别名
- `!` 在控制流中的作用
- `?Sized`

## 要求

1. 用 `UserId(u32)` 演示 newtype
2. 用 `type Kilometers = i32` 演示类型别名
3. 用 `type Thunk = Box<dyn Fn() + Send + 'static>` 减少长类型重复
4. 用 `match + continue` 观察 never type 的效果
5. 写一个 `T: ?Sized` 的函数，支持 `&str`

## 运行方式

```bash
cargo test
```

## 目标行为

- `newtype_can_provide_type_safe_api` 测试通过
- `type_alias_can_reduce_visual_noise` 测试通过
- `question_sized_allows_working_with_str` 测试通过

## 提示

- 类型别名不会创建新类型，只是现有类型的新名字
- newtype 才会带来更强的类型区分和封装效果
