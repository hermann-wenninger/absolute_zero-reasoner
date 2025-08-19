mod compiler;
mod metadata;
mod db;
mod controller;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Absolute-Zero Rust Runtime gestartet...");

    // Controller starten (koordiniert die Multithreads)
    controller::run().await?;

    Ok(())
}
