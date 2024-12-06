/**
 * We had struggles finding ressources with macro component based on the yew framework and we choose to use struct based components.
 * To run the project, you need to have the following dependencies installed:
 * - cargo build
 * - cargo run -p api 
 * - Install python version between 3.8 and 3.11 to be able to use tensorflow
 * - pip install tensorflow
 * - pip install numpy
 * - pip install opencv-python : to be able to use cv2 library for image classification
 * - cd to frontend folder and run the following command:
 * - cargo install trunk 
 * - trunk build  : to build the project to be able to run it
 * - trunk serve --open : pass the --open flag to open the browser automatically
 * 
 */

use gloo::net::http::Request;

use yew::prelude::*;


mod component;
mod types;
use component::labels::SelectLabels;

// all these types are implemented in the types module in types.rs
use types::{ClassificationResult, Image, Msg,App};

// implement the Component trait for App in types module
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            images: vec![],
            classification_result: None,
            selected_file : None,
            uploading: false,
            loading: false,
            selected_labels: String::new(),
            
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
                    self.uploading = true;
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
                self.uploading = true;
                true
            }
            Msg::FileUploaded(Err(err)) => {
                gloo::console::log!("File upload failed: {} ", err);
                // self.loading = ;
                self.uploading = false;
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
                if let Some(file) = self.selected_file.clone() {
                    let convert_file = file.name();
                    if self.selected_labels.is_empty(){
                        self.selected_labels = "None".to_string();
                        gloo::console::log!("Applying all labels because no specific label selected for classification.");
                        // return false;
                    }
                    gloo::console::log!("Lables: {}", self.selected_labels.clone());
                    let url = format!("http://127.0.0.1:8000/classify/{}/{}", convert_file, self.selected_labels);
                    let link = ctx.link().clone();
                    self.loading = true;
                    wasm_bindgen_futures::spawn_local(async move {
                        let response = Request::get(&url).send().await;
                        match response {
                            Ok(res) => {
                                let classification: Result<ClassificationResult, _> = res.json().await.map_err(|e| e.to_string());
                                link.send_message(Msg::ClassificationLoaded(classification));
                            }
                            Err(err) => {
                                gloo::console::log!("Classification failed:", err.to_string());
                            },
                        }
                    });
                } else {
                    gloo::console::log!("No file selected for classification.");
                }
                true
            }
            Msg::ClassificationLoaded(Ok(result)) => {
                self.classification_result = Some(result);
                self.loading = false;
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
            Msg::SelectLabel(label)=>{ // select label to classify
                self.selected_labels.push_str(&label);
                self.selected_labels.push_str(",");
                true
            }

            Msg::ClearSelectLabel=>{ 
                self.selected_labels = String::new();
                gloo::console::log!("Labels cleared ",self.selected_labels.clone());
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="min-h-screen flex flex-col items-center justify-center bg-gradient-to-b from-gray-900 to-gray-800 text-gray-100 font-sans">
                <main class="h-full flex items-center justify-center px-6 sm:py-20 lg:px-8 animate-fadeIn">
                    <div class="text-center">
                        <h1 class="text-balance text-5xl font-extrabold tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-red-600 to-yellow-500 sm:text-7xl animate-textGlow">
                            {"ClassiRust"}
                        </h1>
                        <p class="py-5 text-lg font-medium text-gray-300 sm:text-xl animate-fadeInSlow">
                            {"Classify your Image with confidence using Machine Learning."}
                        </p>
                        
                        <label class="flex flex-col items-center py-3 text-red-600 rounded-lg shadow-lg tracking-wide uppercase border cursor-pointer border-gray-700 bg-gradient-to-r from-gray-800 to-black hover:from-red-700 hover:to-black hover:text-white transform hover:scale-105 transition duration-300 ease-in-out animate-fadeIn">
                            <svg class="w-12 h-12" fill="white" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                                <path d="M16.88 9.94l-4.88-4.88v3.94h-4v-3.94l-4.88 4.88 1.41 1.41 3.47-3.47v3.94h4v-3.94l3.47 3.47 1.41-1.41z"/>
                            </svg>
                            <span class="mt-2 text-xl font-semibold leading-normal">
                                {"Select a file"}
                            </span>
                            <input type="file" accept="image/*" class="hidden" onchange={ctx.link().callback(|e: web_sys::Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let file = input.files().and_then(|files| files.get(0));
                                Msg::FileSelected(file.unwrap())
                            })} />
                        </label>

                        <div id="loading" class="loading mt-4">
                            { if self.loading {
                                html! { <div class="loader text-lg font-bold text-red-500"><i class="fas fa-spinner fa-spin"></i>{" Classifying..."}</div> }
                            } else {
                                html! {}
                            }}
                        </div>

                        <div id="uploading" class="loading mt-4">
                            { if self.uploading {
                                html! { <div class="loader text-lg font-bold text-green-400">{"Image uploaded Successfully"}</div> }
                            } else {
                                html! {}
                            }}
                        </div>  

                        <div class="mt-10 flex flex-col sm:flex-row items-center justify-center gap-x-6"> // use animate-bounce for animation
                            <button onclick={ctx.link().callback(|_| Msg::UploadFile)} class="mt-2 rounded-md bg-red-600 px-5 py-3 text-sm font-bold text-white shadow-lg hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600 transform hover:scale-105 transition duration-300 ease-in-out">
                                { "Upload Image" }
                            </button>

                            <div class="flex flex-row items-center gap-0 w-full sm:w-auto">
                                <SelectLabels on_select={ctx.link().callback(Msg::SelectLabel)} />

                                <button 
                                    onclick={ctx.link().callback(|_| Msg::ClearSelectLabel)} 
                                    class="py-2.5 rounded-tr-md rounded-br-md rounded-tl-* rounded-bl-* bg-red-600 px-3.5 text-sm font-bold text-white shadow-md hover:bg-red-500 transform hover:scale-105 transition duration-300 ease-in-out"
                                >
                                    { "Clear Labels" }
                                </button>
                            </div>

                            <button onclick={ctx.link().callback(|_| Msg::Classify)} class="mt-2 rounded-md bg-indigo-600 px-5 py-3 text-sm font-bold text-white shadow-lg hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 transform hover:scale-105 transition duration-300 ease-in-out">
                                { "Classify Image" }
                            </button>
                        </div>
                    </div>

                </main>

                <div class="  flex flex-col items-center justify-center animate-slideUp">
                    {   
                        if let Some(result) = &self.classification_result {
                            html! {
                                <div class="border border-gray-500 border-dashed rounded-lg shadow-md bg-gradient-to-r from-gray-800 to-gray-900">
                                    <h2 class="text-xl font-bold text-gray-200 mb-2 animate-fadeIn">
                                        { "Classification Result" }
                                    </h2>
                                    <img class="w-200 h-400 rounded shadow-lg" src={result.path.clone()} alt="Classified Image"/>
                                    <div class="text-gray-300">
                                        <ul class="list-disc pl-5">
                                            <li><strong>{"View file:"}</strong> <a href={result.path.clone()} class="text-blue-400 hover:underline">{ &result.path }</a></li>
                                        </ul>
                                    </div> 
                                </div>
                            }
                        } else {
                            html! { <p class="text-gray-600 animate-fadeIn">{ "No classification result yet." }</p> }
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
