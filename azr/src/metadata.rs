use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilationMetadata {
    pub id: Uuid,
    pub project_path: String,
    pub timestamp: i64,
    pub success: bool,
    pub output: String,
}
