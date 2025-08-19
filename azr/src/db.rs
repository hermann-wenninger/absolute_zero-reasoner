use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::{prelude::*, qdrant::*};
use anyhow::Result;

pub async fn init_db() -> Result<QdrantClient> {
    let config = QdrantClientConfig::from_url("http://localhost:6333");
    let client = QdrantClient::new(Some(config))?;

    // Collection erzeugen, falls nicht vorhanden
    client.create_collection(&CreateCollection {
        collection_name: "compilations".into(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 512,
                distance: Distance::Cosine.into(),
            })),
        }),
        ..Default::default()
    }).await?;

    Ok(client)
}
