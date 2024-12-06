use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub path: String, // path to the image file on the file system
}

#[derive(Serialize)]
pub struct ClassificationResult {
    pub label: String,
    pub confidence: f32,
    pub path : PathBuf
}

#[derive(Deserialize)]
pub struct UploadImage {
    pub name: String,
    pub data: Vec<u8>,
}
