
use gloo::net::http::Request;
// use gloo_file::callbacks::FileReader;
use gloo_file::File;
// use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{File as WsFile, FormData};

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
    ClassifyFileSelected(File), // holds a file to be classified from user input
    FileSelected(WsFile), // holds a file to be upload from user input
    UploadFile, // handler to upload a file to backend api
    FileUploaded(Result<String, String>), // holder for 
    ImagesLoaded(Result<Vec<Image>, String>),
    ClassificationLoaded(Result<ClassificationResult, String>),
    LoadImages,
    Classify,
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
                self.selected_file = Some(file.clone());
                gloo::console::log!("File selected: {}", file.name());
                true
            }

            //this is workin file and it is able to send an image to db/backend
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
            Msg::Classify => {
                // let file = self.selected_file.clone().unwrap();
                let file = self.selected_file.clone().unwrap();
                let convert_file = File::from(file);
                let url = format!("http://127.0.0.1:8000/classify/{:?}", convert_file);
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
            <main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
            <div class="text-center">
                <h1 class="mt-4 text-balance text-5xl font-semibold tracking-tight text-red-600 sm:text-7xl">{"ClassiRust"}</h1>
                <p class="mt-6 py-5 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">{"Classify your Image with confidence using Machine Learning."}</p>
                
                <label class="flex flex-col items-center py-7 text-blue rounded-lg shadow-lg tracking-wide uppercase border cursor-pointer border-red bg-gray-100 hover:bg-indigo-500 hover:text-white">
                    <svg class="w-8 h-8" fill="blue" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                        <path d="M16.88 9.94l-4.88-4.88v3.94h-4v-3.94l-4.88 4.88 1.41 1.41 3.47-3.47v3.94h4v-3.94l3.47 3.47 1.41-1.41z"/>
                    </svg>
                    <span class="mt-2 text-2xl font-semibold leading-normal">{"Select a file"}</span>
                    <input type="file" accept="image/*" class="hidden" onchange={ctx.link().callback(|e: web_sys::Event| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        let file = input.files().and_then(|files| files.get(0));
                        Msg::FileSelected(file.unwrap())
                    })} />
                </label>

            
                <div class="mt-10 flex items-center justify-center gap-x-6">
                    <button onclick={ctx.link().callback(|_| Msg::UploadFile)} class="mt-2 rounded-md bg-red-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600">
                    { "Upload Image" }
                </button>
                    <a href="/view/classified.png" class=" mt-3 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{"Classify"}</a>
                    <a href="#" class="text-2xl font-semibold text-gray-900">{"Send feedback.. "}<span aria-hidden="true"></span></a>
                </div>
            </div>
            <ul >

                { for self.images.iter().map(|image| {
                    html! {
                        <li>
                            { format!("You uploaded: {}",image.name) }
                            <button onclick={ctx.link().callback(move |_| Msg::Classify)} class="mt-2 rounded-md bg-red-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600">
                                { "Classify" }
                            </button>
                        </li>
                    }
                })}
            </ul>


        </main>
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
