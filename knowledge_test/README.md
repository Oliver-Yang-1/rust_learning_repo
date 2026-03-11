# Rust 知识点练习

这些练习对应 `book-split-test/chapters/` 下拆分后的章节内容，按主题拆成了多个独立的 `cargo` 项目，方便你逐个练习。

## 目录说明

### 第 1.4 节

- `ch1_4_1_variables`: 变量、可变性、常量、隐藏（shadowing）。
- `ch1_4_2_data_types`: 标量类型、复合类型、类型标注。
- `ch1_4_3_functions`: 函数参数、表达式、返回值。
- `ch1_4_4_comments`: 注释的写法与注释位置。
- `ch1_4_5_control_flow`: `if`、`else if`、`loop`、`while`、`for`。
- `ch1_4_6_overview`: 综合练习，覆盖变量、类型、函数与控制流。

### 第 1.5 节

- `ch1_5_ownership_overview`: 所有权总览，覆盖 move、clone、借用、Copy、slice。
- `ch1_5_1_what_is_ownership`: 所有权基础、作用域、移动、克隆、Copy。
- `ch1_5_2_references_borrowing`: 不可变引用、可变引用、借用规则。
- `ch1_5_3_slices`: `&str`、字符串切片、数组切片。

### 第 1.6 节

- `ch1_6_structs_overview`: 结构体总览，覆盖普通结构体、更新语法、元组结构体。
- `ch1_6_1_defining_structs`: 定义并实例化结构体、字段修改、字段初始化简写。
- `ch1_6_2_rectangle_example`: 用结构体重构长方形面积示例，并派生 `Debug`。
- `ch1_6_3_method_syntax`: 方法、带参数的方法、关联函数。

### 第 1.7 节

- `ch1_7_enums_overview`: 枚举总览，覆盖带数据成员、`Option`、`match`。
- `ch1_7_1_defining_enums`: 定义枚举、成员携带不同数据、枚举方法。
- `ch1_7_2_match_control_flow`: 用 `match` 匹配枚举与 `Option`，以及通配符分支。
- `ch1_7_3_if_let`: 用 `if let` 处理只关心一种模式的情况。

### 第 2.1 节

- `ch2_1_modules_overview`: 模块系统总览，覆盖模块、`pub`、`use`。
- `ch2_1_1_packages_crates`: 包、二进制 crate、库 crate 的关系。
- `ch2_1_2_modules_privacy`: 模块私有性、公有结构体字段、公有枚举。
- `ch2_1_3_paths`: 绝对路径、相对路径、`super`。
- `ch2_1_4_use_keyword`: `use`、标准库导入、别名 `as`。
- `ch2_1_5_split_modules`: 将模块拆到多个文件中。

### 第 2.2 节

- `ch2_2_collections_overview`: 常见集合总览，覆盖 `Vec`、`String`、`HashMap`。
- `ch2_2_1_vectors`: 创建、读取、遍历和修改 `Vec<T>`。
- `ch2_2_2_strings`: `String` 创建、拼接、slice 与 `chars()`。
- `ch2_2_3_hashmaps`: `HashMap` 的插入、读取、覆盖与计数。

### 第 2.3 节

- `ch2_3_error_handling_overview`: 错误处理总览，区分 `Result` 与 `panic!`。
- `ch2_3_1_panic_unrecoverable`: `panic!`、越界与 backtrace 的基本体验。
- `ch2_3_2_result_recoverable`: `Result`、`match`、`?` 与返回 `Result` 的 `main`。
- `ch2_3_3_panic_or_not`: 何时 `panic!`，以及用自定义类型保证值有效。

### 第 2.4 节

- `ch2_4_generics_overview`: 泛型、trait、生命周期总览。
- `ch2_4_1_generic_types`: 泛型函数、泛型结构体与泛型方法。
- `ch2_4_2_traits`: trait 定义、默认实现与 trait bound。
- `ch2_4_3_lifetimes`: 生命周期标注、含引用字段的结构体与相关方法。

### 第 2.5 节

- `ch2_5_testing_overview`: 自动化测试总览，练习 `#[test]` 和 `cargo test`。
- `ch2_5_1_writing_tests`: `assert!`、`assert_eq!`、`should_panic`、返回 `Result` 的测试。
- `ch2_5_2_running_tests`: 过滤测试、显示输出、忽略测试、测试线程参数。
- `ch2_5_3_test_organization`: 单元测试、集成测试与 `tests/common/mod.rs` 组织方式。

### 第 2.6 节

- `ch2_6_io_project_overview`: I/O 命令行项目总览。
- `ch2_6_1_cli_args`: 读取并保存命令行参数。
- `ch2_6_2_reading_files`: 根据参数读取文件内容。
- `ch2_6_3_refactor_config`: 用 `Config` 和 `run` 重构程序结构。
- `ch2_6_4_tdd_search`: 通过测试驱动开发实现 `search`。
- `ch2_6_5_env_vars`: 用环境变量控制大小写敏感搜索。
- `ch2_6_6_stderr`: 将错误信息输出到标准错误。

## 使用方式

1. 进入某个项目目录，比如 `cd knowledge_test/ch1_5_1_what_is_ownership`
2. 先阅读该项目下的 `README.md`
3. 按要求修改 `src/main.rs`
4. 运行 `cargo run`，对照题面中的目标输出检查结果

## 建议顺序

建议先按章节顺序做，再做每章的综合题。
