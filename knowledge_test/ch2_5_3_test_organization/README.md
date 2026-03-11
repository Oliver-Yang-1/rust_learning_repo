# 练习：测试的组织结构

来源章节：`048-2-5-3-测试的组织结构.md`

## 练习目标

练习以下概念：

- 单元测试
- `#[cfg(test)]`
- 测试私有函数
- 集成测试
- `tests/common/mod.rs`

## 要求

1. 阅读 `src/lib.rs` 中的单元测试
2. 确认其中测试了一个私有函数
3. 阅读 `tests/integration_test.rs`
4. 阅读 `tests/common/mod.rs`
5. 运行全部测试
6. 再只运行集成测试文件

## 推荐命令

```bash
cargo test
cargo test --test integration_test
```

## 目标结果

```txt
test result: ok.
```

并且你应该能观察到：

- `src/lib.rs` 中的单元测试会运行
- `tests/integration_test.rs` 中的集成测试也会运行
- `tests/common/mod.rs` 不会被当作单独测试文件显示为 `running 0 tests`

## 提示

- 单元测试和源码放在一起
- 集成测试放在 `tests/` 目录
- 公共测试辅助函数建议放在 `tests/common/mod.rs`
