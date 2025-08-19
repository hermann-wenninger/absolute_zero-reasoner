use crate::compiler::compile_project;
use crate::metadata::CompilationMetadata;
use crate::db::init_db;
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;
use tokio::task;

pub async fn run() -> Result<()> {
    let client = init_db().await?;

    // Beispiel: mehrere Projekte parallel kompilieren
    let projects = vec![
        "examples/project1",
        "examples/project2",
    ];

    let mut handles = vec![];

    for project in projects {
        let project_path = project.to_string();
        handles.push(task::spawn(async move {
            match compile_project(&project_path).await {
                Ok(out) => CompilationMetadata {
                    id: Uuid::new_v4(),
                    project_path,
                    timestamp: Utc::now().timestamp(),
                    success: true,
                    output: out,
                },
                Err(e) => CompilationMetadata {
                    id: Uuid::new_v4(),
                    project_path,
                    timestamp: Utc::now().timestamp(),
                    success: false,
                    output: e.to_string(),
                },
            }
        }));
    }

    for handle in handles {
        let result = handle.await?;
        println!("🔧 Kompilation abgeschlossen: {:?}", result);
        // TODO: in Qdrant speichern
    }

    Ok(())
}
