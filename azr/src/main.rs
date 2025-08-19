mod compiler;
mod metadata;
mod db;
mod controller;

//use anyhow::Result;
use qdrant_client::Qdrant;
use qdrant_client::config::QdrantConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Config bauen
    let config = QdrantConfig::from_url("http://localhost:6333");
    
    // Client erstellen
    let _client = Qdrant::new(config)?;

    // Beispiel: Client-Info ausgeben
    println!("Qdrant-Client verbunden!");

    Ok(())
}