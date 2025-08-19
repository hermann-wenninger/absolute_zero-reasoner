
  
use qdrant_client::Qdrant;
use qdrant_client::config::QdrantConfig; // <-- richtige Imports

pub async fn init_db() -> anyhow::Result<Qdrant> {
    // URL ggf. aus ENV lesen
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".into());

    // Config bauen (optional mit API-Key etc.)
    let config = QdrantConfig::from_url(&url);
    // .with_api_key("dein_api_key") // falls nötig

    let client = Qdrant::new(config)?;
    Ok(client)
}
