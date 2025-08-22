use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..]);
        println!("Anfrage erhalten:\n{}", request);

        // Erste Zeile extrahieren (z.B. "GET /hello HTTP/1.1")
        let request_line = request.lines().next().unwrap_or("");
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");

        // Default-Body (falls nichts passt)
        let mut status = "200 OK";
        let mut body = "<h1>Seite nicht gefunden</h1>";
        let mut content_type = "text/html; charset=UTF-8";

        // Router: Methode + Pfad
        match (method, path) {
            ("GET", "/") => {
                body = "<h1>Willkommen auf der Startseite!</h1>";
            }
            ("GET", "/hello") => {
                body = "<h1>Hallo!</h1>";
            }
            ("GET", "/json") => {
                body = r#"{ "status": "ok", "message": "Hallo aus JSON" }"#;
                content_type = "application/json; charset=UTF-8";
            }
            _ => {
                status = "404 Not Found";
            }
        }

        let response = format!(
            "HTTP/1.1 {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            status,
            content_type,
            body.len(),
            body
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Konnte Port nicht binden");
    println!("Server läuft auf http://127.0.0.1:7878 so dahin");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Fehler: {}", e),
        }
    }
}
