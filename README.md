# learning_rust

这是一个围绕 Rust 学习内容整理出来的练习型项目。

项目的核心目标不是只保存笔记，而是把书里的知识点拆成可以直接动手运行的 `cargo` 小项目，方便按章节学习、练习和复盘。

## 项目结构

### `full.md`

完整的原始学习资料汇总文件。

### `book-split-test/`

把学习资料按章节拆分后的版本，便于逐章阅读和定位内容。

主要内容包括：

- `chapters/`：按章节拆分后的 Markdown 文件
- `toc.md`：目录
- `body-prefix.md`、`body-headings.md`：拆分过程中的辅助文件
- `manifest.json`：章节清单与相关元数据

如果你想先看概念来源，再做题，建议先从这里开始。

### `knowledge_test/`

按知识点拆出来的 Rust 练习工程集合。这里的每个目录基本都是一个独立的 `cargo` 项目，通常包含：

- `README.md`：题目说明、目标、要求、运行方式、预期输出
- `src/main.rs` 或 `src/lib.rs`：练习代码
- 必要时附带测试文件、示例输入文件或静态资源

这些练习覆盖了从基础语法到进阶主题的内容，目前已经整理到 `4-4` 附录，包括：

- 基础语法：变量、数据类型、函数、控制流
- 所有权：move、borrow、slice
- 结构体、枚举、模式匹配
- 模块、包、crate、集合、错误处理
- 泛型、trait、生命周期、测试
- I/O 项目与命令行程序
- 闭包、迭代器、Cargo、工作空间
- 智能指针、并发、面向对象特性
- 模式、unsafe、宏、Web Server
- 附录里的可操作主题，例如原始标识符、运算符、可派生 trait、开发工具

更细的练习导航可以看 `knowledge_test/README.md`。

## 适合怎么用

推荐按下面的顺序学习：

1. 先阅读 `book-split-test/chapters/` 里的对应章节
2. 再进入 `knowledge_test/` 中同主题的练习项目
3. 先看该项目自己的 `README.md`
4. 自己实现或修改代码
5. 运行 `cargo run` 或 `cargo test` 验证结果

如果你只是想快速刷题，也可以直接从 `knowledge_test/README.md` 里挑感兴趣的主题开始。

## 快速开始

例如，进入一个练习项目：

```bash
cd knowledge_test/ch1_4_1_variables
cargo run
```

再比如，运行一个以测试为主的练习：

```bash
cd knowledge_test/ch4_4_c_derivable_traits
cargo test
```

## 命名规则

`knowledge_test/` 下的项目一般使用如下命名方式：

```txt
chX_Y_Z_topic
```

含义大致是：

- `X`：大章节
- `Y`：小节
- `Z`：当前小节下的知识点序号
- `topic`：该练习的主题名

例如：

- `ch1_4_1_variables`
- `ch3_1_2_iterators`
- `ch4_3_2_thread_pool`

## 说明

- 各练习项目彼此独立，通常不共享一个 workspace
- 很多题目刻意设计成“小而完整”，方便单独运行和反复实验
- 一部分项目更适合用 `cargo run`，另一部分更适合用 `cargo test`
- 附录中偏参考性质的内容未必都会被做成练习，优先选择适合动手验证的主题

## 下一步建议

如果你第一次打开这个项目，建议直接从这两个入口开始：

- 阅读 `knowledge_test/README.md`，看完整练习导航
- 进入 `knowledge_test/ch1_4_1_variables`，从最基础的题目开始跑通
# rust_learning_repo
