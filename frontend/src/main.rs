use gloo::net::http::Request;
use gloo_file::callbacks::FileReader;
use gloo_file::File;
// use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
    FileSelected(File),
    FileUploaded(Result<String, String>),
    ImagesLoaded(Result<Vec<Image>, String>),
    ClassificationLoaded(Result<ClassificationResult, String>),
    LoadImages,
    Classify(String),
    UploadImage(Image),
    ImageUploaded(Result<String, String>),
}

struct App {
    file_reader: Option<FileReader>,
    images: Vec<Image>,
    classification_result: Option<ClassificationResult>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            file_reader: None,
            images: vec![],
            classification_result: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileSelected(file) => {
                let file_name = file.name().clone();
                let link = ctx.link().clone();

                let reader = gloo_file::callbacks::read_as_bytes(&file, move |file_result| {
                    let result = file_result.map(|_| "File read successfully".to_string())
                                             .map_err(|err| err.to_string());
                    link.send_message(Msg::FileUploaded(result));
                });
                self.file_reader = Some(reader);
                true
            }
            Msg::FileUploaded(Ok(message)) => {
                gloo::console::log!("File uploaded:", message);
                ctx.link().send_message(Msg::LoadImages);
                true
            }
            Msg::FileUploaded(Err(err)) => {
                gloo::console::log!("File upload failed:", err);
                false
            }
            Msg::ImagesLoaded(Ok(images)) => {
                self.images = images;
                true
            }
            Msg::ImagesLoaded(Err(err)) => {
                gloo::console::log!("Failed to load images:", err);
                false
            }
            Msg::Classify(file_path) => {
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
            Msg::UploadImage(image) => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::post("http://127.0.0.1:8000/upload/")
                        .json(&image) // Send image metadata
                        .unwrap()
                        .send()
                        .await;

                    match response {
                        Ok(_) => link.send_message(Msg::ImageUploaded(Ok("Image uploaded successfully!".to_string()))),
                        Err(err) => link.send_message(Msg::ImageUploaded(Err(err.to_string()))),
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
                <h1>{ "Rust Classy - Image Upload and Classification" }</h1>

                <input type="file" onchange={ctx.link().callback(|e: Event| {
                    let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                    let file = input.files().unwrap().get(0).unwrap();
                    let file = File::from(file);
                    Msg::FileSelected(file)
                })} />

                <button onclick={ctx.link().callback(|_| Msg::LoadImages)}>{ "Load Images" }</button>

                <h2>{ "Uploaded Images" }</h2>
                <ul>
                    { for self.images.iter().map(|image| {
                        let file_path = image.path.clone();
                        html! {
                            <li>
                                { format!("ID: {} Name: {}", image.id, image.name) }
                                <button onclick={ctx.link().callback(move |_| Msg::Classify(file_path.clone()))}>
                                    { "Classify" }
                                </button>
                            </li>
                        }
                    })}
                </ul>

                {
                    if let Some(result) = &self.classification_result {
                        html! {
                            <div>
                                <h2>{ "Classification Result" }</h2>
                                <p>{ format!("Label: {} Confidence: {:.2}%", result.label, result.confidence * 100.0) }</p>
                            </div>
                        }
                    } else {
                        html! { <p>{ "No classification result yet." }</p> }
                    }
                }
            </div>
        }
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
