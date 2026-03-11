# 练习：单线程 Web Server

来源章节：`096-4-3-1-单线程-web-server.md`

## 练习目标

练习以下概念：

- 读取请求行
- 判断 `GET /` 与其他路径
- 生成 HTTP 状态行
- 返回不同的 HTML 页面

## 要求

1. 实现一个根据请求字节生成响应字符串的函数
2. 对 `GET / HTTP/1.1` 返回 `200 OK`
3. 对其他路径返回 `404 NOT FOUND`
4. 分别从 `hello.html` 和 `404.html` 读取页面内容

## 运行方式

```bash
cargo run
cargo test
```

## 目标输出

```txt
root status: HTTP/1.1 200 OK
missing status: HTTP/1.1 404 NOT FOUND
```

## 提示

- 这里先练“请求到响应”的核心逻辑，不要求真的长时间监听端口
- 真实 server 只是把这里的逻辑放到 `TcpStream` 读写流程中
