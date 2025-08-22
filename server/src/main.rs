use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Anfrage lesen
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Konvertieren in String (nur Debug-Zwecke, unsauber bei Binärdaten!)
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("Anfrage erhalten:\n{}", request);

            // Eine ganz einfache HTTP-Response
            let response = "HTTP/1.1 200 OK\r\n\
                            Content-Type: text/plain; charset=UTF-8\r\n\
                            Content-Length: 13\r\n\
                            \r\n\
                            Hallo, Welt!";

            // Antwort senden
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => eprintln!("Fehler beim Lesen: {}", e),
    }
}

fn main() {
    // TCP-Socket auf Port 7878 öffnen
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Konnte Port nicht binden");

    println!("Server läuft auf http://127.0.0.1:7878 ...");

    // Eingehende Verbindungen akzeptieren
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => eprintln!("Verbindungsfehler: {}", e),
        }
    }
}
