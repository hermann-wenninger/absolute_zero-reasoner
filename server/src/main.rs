use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..]);
        println!("Anfrage erhalten:\n{}", request);

        // Hier ein einfacher HTML-Body
        let body = "<!DOCTYPE html><html><body><h1>Hallo, Welt!</h1></body></html>";

        // Content-Length berechnen (in Bytes)
        let length = body.len();

        // Vollständige HTTP-Antwort mit Headern
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=UTF-8\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            length, body
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        // Verbindung wird durch `Connection: close` und stream-Ende sauber beendet
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Konnte Port nicht binden");
    println!("Server läuft auf http://127.0.0.1:7878 ...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Fehler: {}", e),
        }
    }
}
