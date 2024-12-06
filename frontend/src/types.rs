use serde::{Deserialize, Serialize};
use web_sys::File as WsFile;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClassificationResult {
    pub label: String,
    pub confidence: f32,
    pub path: String,
}

pub enum Msg {
    FileSelected(WsFile), // holds a file to be upload from user input
    UploadFile, // handler to upload a file to backend api
    FileUploaded(Result<String, String>), // holder for 
    ImagesLoaded(Result<Vec<Image>, String>),
    ClassificationLoaded(Result<ClassificationResult, String>),
    LoadImages,
    Classify,
    SelectLabel(String),
    ClearSelectLabel,
    //web socket 


}

pub struct App {
    // file_reader: Option<gloo_file::callbacks::FileReader>,
    pub images: Vec<Image>,
    pub classification_result: Option<ClassificationResult>,
    pub selected_file: Option<WsFile>,
    pub loading: bool,
    pub uploading:bool,
    pub selected_labels: String,
}

