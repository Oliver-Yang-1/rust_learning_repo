# 练习：读取文件

来源章节：`051-2-6-2-读取文件.md`

## 练习目标

练习以下概念：

- `std::fs::read_to_string`
- 从命令行参数中读取文件名
- 打印文件内容

## 要求

1. 从命令行读取查询词和文件名
2. 用 `read_to_string` 读取文件内容
3. 打印查询词
4. 打印文件名
5. 打印完整文本内容

## 运行方式

```bash
cargo run the poem.txt
```

## 目标输出

```txt
searching for: the
in file: poem.txt
with text:
I'm nobody! Who are you?
...
```

## 提示

- 这一题先专注文件读取，还不需要真正实现搜索
- 示例文件已经放在项目根目录的 `poem.txt`
