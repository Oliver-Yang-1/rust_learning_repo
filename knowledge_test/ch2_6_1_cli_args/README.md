# 练习：接受命令行参数

来源章节：`050-2-6-1-接受命令行参数.md`

## 练习目标

练习以下概念：

- `std::env::args`
- `collect`
- 从参数列表中取值
- 将参数保存到变量

## 要求

1. 读取所有命令行参数到 `Vec<String>`
2. 将第 1 个参数保存到 `query`
3. 将第 2 个参数保存到 `filename`
4. 打印参数总数
5. 打印 `query` 和 `filename`

## 运行方式

```bash
cargo run test sample.txt
```

## 目标输出

```txt
arg count: 3
searching for: test
in file: sample.txt
```

## 提示

- 第 0 个参数通常是程序自身路径
- 真正的两个业务参数从索引 `1` 和 `2` 开始
- 这个练习先不处理参数缺失错误
