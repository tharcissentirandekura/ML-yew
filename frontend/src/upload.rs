use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use log;
use gloo_file::{Blob, callbacks::read_as_bytes};

mod component;
use component::navbar::Navbar;

#[derive(Deserialize, Clone, Debug)]
struct ClassificationResponse {
    label: String,
    confidence: f32,
}

#[function_component(App)]
fn app() -> Html {
    let file_data = use_state(|| None); // To hold the image file data
    let classification = use_state(|| None); // To store the classification result

    let on_file_change = {
        let file_data = file_data.clone();
        Callback::from(move |e: web_sys::Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let file = files.get(0).unwrap(); // Get the first file (assuming only one file is uploaded)
                let file_data = file_data.clone();

                let blob = Blob::from(file);

                // Read file data as binary
                read_as_bytes(&blob, move |res| {
                    match res {
                        Ok(data) => file_data.set(Some(data)), // Set binary data to state
                        Err(err) => log::error!("Error reading file: {:?}", err),
                    }
                });
            }
        })
    };


    let on_submit = {
        let file_data = file_data.clone();
        let classification = classification.clone();

        Callback::from(move |_| {
            let file_data = file_data.clone();
            let classification = classification.clone();

            if let Some(data) = (*file_data).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    let request = Request::post("http://127.0.0.1:8000/upload/")
                        .header("Content-Type", "application/octet-stream")
                        .body(data);

                    let response = match request {
                        Ok(request) => request.send().await.unwrap().json::<ClassificationResponse>().await.unwrap(),
                        Err(e) => {
                            log::error!("Failed to create request: {:?}", e);
                            return;
                        }
                    };

                    classification.set(Some(response));
                });
            }
        })
    };
    html! {
        <main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
            <div class="text-center">
                <h1 class="mt-4 text-balance text-5xl font-semibold tracking-tight text-red-600 sm:text-7xl">{"ClassiRust"}</h1>
                <p class="mt-6 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">{"Classify your Image with confidence using Machine Learning."}</p>
                <form class="p-10 bg-sky-200" onsubmit={on_submit.clone()}>
                    <label class="flex flex-col items-center py-7 text-blue rounded-lg shadow-lg tracking-wide uppercase border cursor-pointer border-red bg-gray-100 hover:bg-indigo-500 hover:text-white">
                        <svg class="w-8 h-8" fill="blue" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                            <path d="M16.88 9.94l-4.88-4.88v3.94h-4v-3.94l-4.88 4.88 1.41 1.41 3.47-3.47v3.94h4v-3.94l3.47 3.47 1.41-1.41z"/>
                        </svg>
                        <span class="mt-2 text-2xl font-semibold leading-normal">{"Select a file"}</span>
                        <input
                            type="file"
                            class="hidden"
                            onchange={on_file_change.clone()}
                        />
                    </label>
                    <input type="submit" value="Submit" class="mt-4 rounded-md bg-red-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600" />
                </form>
                <div class="mt-10 flex items-center justify-center gap-x-6">
                    <a href="/view/classified.png" class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{"Classify"}</a>
                    <a href="#" class="text-2xl font-semibold text-gray-900">{"Send feedback.. "}<span aria-hidden="true"></span></a>
                </div>
            </div>
        </main>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
