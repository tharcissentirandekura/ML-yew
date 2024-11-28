
use gloo::net::http::Request;
// use gloo_file::callbacks::FileReader;
// use gloo_file::File;
// use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::File as WsFile;

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
    FileSelected(WsFile), // holds a file to be upload from user input
    UploadFile, // handler to upload a file to backend api
    FileUploaded(Result<String, String>), // holder for 
    ImagesLoaded(Result<Vec<Image>, String>),
    ClassificationLoaded(Result<ClassificationResult, String>),
    LoadImages,
    Classify,
    // UploadImage(WsFile),
    // ImageUploaded(Result<String, String>),
}

struct App {
    // file_reader: Option<gloo_file::callbacks::FileReader>,
    images: Vec<Image>,
    classification_result: Option<ClassificationResult>,
    selected_file: Option<WsFile>,
    loading: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // file_reader: None,
            images: vec![],
            classification_result: None,
            selected_file : None,
            loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::FileSelected(file) => {
                self.selected_file = Some(file.clone());
                gloo::console::log!("File selected: {}", file.name());
                true
            }

            //this is workin file and it is able to send an image to db/backend
            Msg::UploadFile => {
                if let Some(file) = self.selected_file.clone() {
                    self.loading = true;
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
                        // self.loading = false;
                        
                    });
                } else {
                    gloo::console::log!("No file selected for upload.");
                }
                false
            }


            Msg::FileUploaded(Ok(message)) => {
                gloo::console::log!("File uploaded successfully: {}", message);
                ctx.link().send_message(Msg::LoadImages);
                self.loading = false;
                true
            }
            Msg::FileUploaded(Err(err)) => {
                gloo::console::log!("File upload failed: {} ", err);
                self.loading = false;
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
            Msg::Classify => {
                // let file = self.selected_file.clone().unwrap();
                let file = self.selected_file.clone().unwrap();
                let convert_file = file.name();
                let url = format!("http://127.0.0.1:8000/classify/{}", convert_file);
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
                    let response = Request::get("http://127.0.0.1:800/image").send().await;
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

            // Msg::ImageUploaded(Ok(message)) => {
            //     gloo::console::log!(message); // Log success message
            //     true
            // }
            // Msg::ImageUploaded(Err(err)) => {
            //     gloo::console::log!("Failed to upload image:", err); // Log error message
            //     false
            // }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="grid min-h-full place-items-center bg-gray-900 px-6 sm:py-32 lg:px-8 txt-gray-300">
                <main class=" grid min-h-full place-items-center bg-gray-900 px-6 sm:py-3 lg:px-8">
                    <div class="text-center">
                        <h1 class="text-balance text-5xl font-semibold tracking-tight text-red-600 sm:text-7xl">{"ClassiRust"}</h1>
                        <p class="py-5 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">{"Classify your Image with confidence using Machine Learning."}</p>
                        
                        <label class="flex flex-col items-center py-2 text-red-600 rounded-lg shadow-lg tracking-wide uppercase border cursor-pointer border-gray-600 bg-gray-800 hover:bg-black hover:text-white">
                            <svg class="w-10 h-8" fill="white" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                                <path d="M16.88 9.94l-4.88-4.88v3.94h-4v-3.94l-4.88 4.88 1.41 1.41 3.47-3.47v3.94h4v-3.94l3.47 3.47 1.41-1.41z"/>
                            </svg>
                            <span class="mt-1 text-xl font-semibold leading-normal">{"Select a file"}</span>
                            <input type="file" accept="image/*" class="hidden" onchange={ctx.link().callback(|e: web_sys::Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let file = input.files().and_then(|files| files.get(0));
                                Msg::FileSelected(file.unwrap())
                            })} />
                        </label>

                        <div id="loading" class="loading">
                            { if self.loading {
                                html! { <div class="loader">{"Loading..."}</div> }
                            } else {
                                html! {}
                            }}
                        </div> 
                        <div class="mt-10 flex items-center justify-center gap-x-6">
                            <button onclick={ctx.link().callback(|_| Msg::UploadFile)} class="mt-2 rounded-md bg-red-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600">
                            { "Upload Image" }
                        </button>

                        <button onclick={ctx.link().callback(|_| Msg::Classify)} class="mt-2 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                            { "Classify Image" }
                        </button>
                        </div>
                    </div>

                </main>
                <div>
                    {   
                        if let Some(result) = &self.classification_result {
                            html! {
                                <div class="mt-10 p-4 mb-4 border border-gray-300 border-dashed rounded-lg shadow-md bg-gray-800">
                                    <h2 class="text-xl font-semibold text-gray-200 mb-2">{ "Classification Result" }</h2>
                                    <img class="w-100 h-auto mb-2 rounded" src={result.path.clone()} alt="Classified Image"/>
                                    <div class="text-gray-700">
                                        <ul class="list-disc pl-5 text-gray-200">
                                            <li><strong>{"Label:"}</strong> { &result.label }</li>
                                            <li><strong>{"Confidence:"}</strong> { format!("{:.2}%", result.confidence * 100.0) }</li>
                                            <li><strong>{"View file:"}</strong> <a href={result.path.clone()} class="text-blue-500 hover:underline">{ &result.path }</a></li>
                                        </ul>
                                    </div> 
                                </div>
                            }
                        }else{
                            html! { <p class="text-gray-600">{ "No classification result yet." }</p> }
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
