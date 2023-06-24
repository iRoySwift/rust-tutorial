use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // 多线程
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });

        // 线程池
        pool.execute(|| {
            handle_connection(stream);
        });
        println!("Shutting down");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //512字节

    stream.read(&mut buffer).unwrap();

    // 请求
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    // 响应
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    let get = b"GET / HTTP/1.1\r\n"; // b 将后面文本转成字节字符串
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Request: {:?}", String::from_utf8_lossy(&buffer[..]));
}
