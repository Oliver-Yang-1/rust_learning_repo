use std::fs;

fn build_response(request: &[u8]) -> String {
    let get_root = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if request.starts_with(get_root) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    format!("{status_line}\r\n\r\n{contents}")
}

fn main() {
    // 任务 1：根据请求行判断是返回首页还是 404 页面。
    let ok_request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let not_found_request = b"GET /missing HTTP/1.1\r\nHost: localhost\r\n\r\n";

    let ok_response = build_response(ok_request);
    let not_found_response = build_response(not_found_request);

    println!("root status: {}", ok_response.lines().next().unwrap());
    println!("missing status: {}", not_found_response.lines().next().unwrap());
}

#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn root_request_returns_200() {
        let response = build_response(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains("Hi from Rust"));
    }

    #[test]
    fn unknown_request_returns_404() {
        let response = build_response(b"GET /foo HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(response.starts_with("HTTP/1.1 404 NOT FOUND"));
        assert!(response.contains("Oops!"));
    }
}
