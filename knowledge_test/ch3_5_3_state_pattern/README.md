# 练习：状态模式

来源章节：`083-3-5-3-面向对象设计模式的实现.md`

## 练习目标

练习以下概念：

- 状态模式
- trait 对象封装状态转换
- 将状态编码进类型
- 用 Rust 类型系统限制非法状态

## 要求

1. 阅读 `object_style` 模块，理解经典状态模式如何工作
2. 运行测试，确认草稿和审核中都不能读取内容
3. 阅读 `typed_style` 模块，理解“把状态编码进类型”的做法
4. 对比两种写法的取舍

## 运行方式

```bash
cargo test
```

## 目标行为

- `object_style_post_only_shows_content_after_approval` 测试通过
- `typed_style_workflow_reaches_published_post` 测试通过

## 提示

- `object_style` 更接近传统面向对象状态模式
- `typed_style` 更符合 Rust 风格，因为非法状态转换会变成编译期问题
