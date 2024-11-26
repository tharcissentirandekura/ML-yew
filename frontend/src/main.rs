
use gloo::net::http::Request;
use gloo_file::{callbacks::FileReader, Blob};
use gloo_file::File;
// use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{Event, HtmlInputElement, File as WsFile, FormData};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Image {
    id: i32,
    name: String,
    path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ClassificationResult {
    label: String,
    confidence: f32,
    path: String,
}

enum Msg {
    ClassifyFileSelected(File),
    FileSelected(WsFile),
    UploadFile,
    FileUploaded(Result<String, String>),
    ImagesLoaded(Result<Vec<Image>, String>),
    ClassificationLoaded(Result<ClassificationResult, String>),
    LoadImages,
    Classify(String),
    UploadImage(WsFile),
    ImageUploaded(Result<String, String>),
}

struct App {
    file_reader: Option<gloo_file::callbacks::FileReader>,
    images: Vec<Image>,
    classification_result: Option<ClassificationResult>,
    selected_file: Option<WsFile>
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            file_reader: None,
            images: vec![],
            classification_result: None,
            selected_file : None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::ClassifyFileSelected(file)=>{
                let link = ctx.link().clone();

                let gloo_file = gloo_file::File::from(file.clone());
                
                let reader = gloo_file::callbacks::read_as_bytes(&gloo_file, move |file_result| {
                    let result = file_result.map(|_| "File read successfully".to_string())
                                             .map_err(|err| err.to_string());
                    link.send_message(Msg::FileUploaded(result));
                    gloo::console::log!("File being tried to upload {}", file.clone().name());
                });

                self.file_reader = Some(reader);
                true
            }

            Msg::FileSelected(file) => {
                // let file_name = file.name().clone();
                self.selected_file = Some(file.clone());
                gloo::console::log!("File selected: {}", file.name());
                true
            }
            Msg::UploadFile => {
                if let Some(file) = self.selected_file.clone() {
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let url = "http://127.0.0.1:8000/upload";
                        let form_data = web_sys::FormData::new().expect("Failed to create FormData");
                        form_data
                            .append_with_blob_and_filename("file", &file, &file.name())
                            .expect("Failed to append file");
            
                        // Do not set the Content-Type header; let the browser handle it
                        let request = gloo_net::http::Request::post(&url)
                            .body(form_data)
                            .expect("Failed to build request");
            
                        match request.send().await {
                            Ok(response) => {
                                if response.ok() {
                                    let text = response.text().await.unwrap_or_default();
                                    link.send_message(Msg::FileUploaded(Ok(text)));
                                } else {
                                    let status = response.status();
                                    let error_msg = format!("Failed with status: {}", status);
                                    link.send_message(Msg::FileUploaded(Err(error_msg)));
                                }
                            }
                            Err(error) => {
                                let error_msg = format!("Request error: {:?}", error);
                                link.send_message(Msg::FileUploaded(Err(error_msg)));
                            }
                        }
                    });
                } else {
                    gloo::console::log!("No file selected for upload.");
                }
                false
            }
            Msg::FileUploaded(Ok(message)) => {
                gloo::console::log!("File uploaded successfully: {}", message);
                ctx.link().send_message(Msg::LoadImages);
                true
            }
            Msg::FileUploaded(Err(err)) => {
                gloo::console::log!("File upload failed: {} ", err);
                false
            }
            Msg::ImagesLoaded(Ok(images)) => {
                self.images = images;
                true
            }
            Msg::ImagesLoaded(Err(err)) => {
                gloo::console::log!("Failed to load images: {} ", err);
                false
            }
            Msg::Classify(file_path) => {
                // let file = self.selected_file.clone().unwrap();


                let url = format!("http://127.0.0.1:8000/classify/{}", file_path);
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get(&url).send().await;
                    match response {
                        Ok(res) => {
                            let classification: Result<ClassificationResult, _> = res.json().await.map_err(|e| e.to_string());
                            link.send_message(Msg::ClassificationLoaded(classification));
                        }
                        Err(err) => gloo::console::log!("Classification failed:", err.to_string()),
                    }
                });
                true
            }
            Msg::ClassificationLoaded(Ok(result)) => {
                self.classification_result = Some(result);
                true
            }
            Msg::ClassificationLoaded(Err(err)) => {
                gloo::console::log!("Failed to load classification result:", err);
                false
            }
            Msg::LoadImages => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("http://127.0.0.1:8000/image").send().await;
                    match response {
                        Ok(res) => {
                            let images: Result<Vec<Image>, String> = res.json().await.map_err(|e| e.to_string());
                            link.send_message(Msg::ImagesLoaded(images));
                        }
                        Err(err) => gloo::console::log!("Failed to load images:", err.to_string()),
                    }
                });
                true
            }

            Msg::UploadImage(file) => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    let url = "http://127.0.0.1:8000/upload";
                    let form_data = FormData::new().expect("Failed to create FormData");
                    form_data
                        .append_with_blob_and_filename("file", &file, &file.name())
                        .expect("Failed to append file");
            
                    // Do not set Content-Type header; let the browser handle it
                    let request = Request::post(&url)
                        .body(form_data)
                        .expect("Failed to build request");
            
                    match request.send().await {
                        Ok(response) => {
                            if response.ok() {
                                let text = response.text().await.unwrap_or_default();
                                link.send_message(Msg::ImageUploaded(Ok(text)));
                            } else {
                                let status = response.status();
                                let error_msg = format!("Failed with status: {}", status);
                                link.send_message(Msg::ImageUploaded(Err(error_msg)));
                            }
                        }
                        Err(error) => {
                            let error_msg = format!("Request error: {:?}", error);
                            link.send_message(Msg::ImageUploaded(Err(error_msg)));
                        }
                    }
                });
                false
            }
            Msg::ImageUploaded(Ok(message)) => {
                gloo::console::log!(message); // Log success message
                true
            }
            Msg::ImageUploaded(Err(err)) => {
                gloo::console::log!("Failed to upload image:", err); // Log error message
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <input type="file" accept="image/*" onchange={ctx.link().callback(|e: web_sys::Event| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    let file = input.files().and_then(|files| files.get(0));
                    Msg::FileSelected(file.unwrap())
                })} />
                <button onclick={ctx.link().callback(|_| Msg::UploadFile)}>
                    { "Upload Image" }
                </button>

                // <button onclick={ctx.link().callback(|_| Msg::Classify(""))}>
            //     { "Upload Image" }
            // // </button>

                <div>
                    {
                        if let Some(result) = &self.classification_result {
                            html! {
                                
                                <div>
                                    <h2>{ "Classification Result" }</h2>
                                    <p>{ format!("Label: {} Confidence: {:.2}%", result.label, result.confidence * 100.0) }</p>
                                </div>
                            }  
                        }else{
                            html! { <p>{ "No classification result yet." }</p> }
                        }
                    }

                </div>

            </div>
        }
    }

}


fn main() {
    yew::Renderer::<App>::new().render();
}
