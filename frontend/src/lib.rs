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
                    let request = Request::post("http://127.0.0.1:8080/api/classify")
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
        <div class="body fs-4">
            <Navbar onclick={on_submit.clone()}/> // Include the Navbar component
            <h1 class="text text-danger p-4 m-4">{ "Machine Learning Image Classifier" }</h1>
            <input
                type="file"
                accept="image/*"
                onchange={on_file_change}
            />
            <button onclick={on_submit}>{ "Classify Image" }</button>
            <p>
                {
                    if let Some(classification) = &*classification {
                        html! { <>{ format!("Label: {}, Confidence: {:.2}%", classification.label, classification.confidence * 100.0) }</> }
                    } else {
                        html! { "Upload an image to get started." }
                    }
                }
            </p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
