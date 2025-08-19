use anyhow::Result;
use tokio::process::Command;
use std::path::Path;

pub async fn compile_project(path: &str) -> Result<String> {
    let project_path = Path::new(path);

    let output = Command::new("cargo")
        .arg("build")
        .current_dir(project_path)
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow::anyhow!(
            "Kompilierung fehlgeschlagen: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
