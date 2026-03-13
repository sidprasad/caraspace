use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process;

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/rust_viz_data.html".to_string());
    let port = env::var("SPYTIAL_PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(8080);

    if !Path::new(&file_path).exists() {
        eprintln!(
            "Visualization file does not exist: {}. Run a diagram example first.",
            file_path
        );
        process::exit(1);
    }

    let bind_addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&bind_addr).unwrap_or_else(|error| {
        eprintln!("Failed to bind {}: {}", bind_addr, error);
        process::exit(1);
    });

    println!("Visualization server ready at http://localhost:{port}/rust_viz_data.html");
    println!("Health endpoint at http://localhost:{port}/health");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream, &file_path),
            Err(error) => eprintln!("Connection error: {}", error),
        }
    }
}

fn handle_connection(stream: &mut std::net::TcpStream, file_path: &str) {
    let mut buffer = [0_u8; 4096];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(read) => read,
        Err(error) => {
            eprintln!("Failed to read request: {}", error);
            return;
        }
    };

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut parts = request
        .lines()
        .next()
        .unwrap_or_default()
        .split_whitespace();

    let method = parts.next().unwrap_or_default();
    let raw_path = parts.next().unwrap_or("/");
    let path = raw_path.split('?').next().unwrap_or(raw_path);

    if method != "GET" {
        write_response(
            stream,
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            b"Method Not Allowed",
        );
        return;
    }

    match path {
        "/" => {
            let headers = "HTTP/1.1 302 Found\r\nLocation: /rust_viz_data.html\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            if let Err(error) = stream.write_all(headers.as_bytes()) {
                eprintln!("Failed to write redirect response: {}", error);
            }
        }
        "/health" => write_response(stream, "200 OK", "text/plain; charset=utf-8", b"ok"),
        "/favicon.ico" => {
            write_response(stream, "204 No Content", "text/plain; charset=utf-8", b"")
        }
        "/rust_viz_data.html" => match fs::read(file_path) {
            Ok(contents) => write_response(stream, "200 OK", "text/html; charset=utf-8", &contents),
            Err(error) => {
                let body = format!("Failed to read visualization file: {}", error);
                write_response(
                    stream,
                    "500 Internal Server Error",
                    "text/plain; charset=utf-8",
                    body.as_bytes(),
                );
            }
        },
        _ => write_response(
            stream,
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"Not Found",
        ),
    }
}

fn write_response(stream: &mut std::net::TcpStream, status: &str, content_type: &str, body: &[u8]) {
    let headers = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );

    if let Err(error) = stream.write_all(headers.as_bytes()) {
        eprintln!("Failed to write response headers: {}", error);
        return;
    }

    if let Err(error) = stream.write_all(body) {
        eprintln!("Failed to write response body: {}", error);
    }
}
