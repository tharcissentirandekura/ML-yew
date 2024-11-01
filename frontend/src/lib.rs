use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
#[derive(Deserialize, Clone, Debug)]
struct GreetResponse {
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let name = use_state(|| "World".to_string());
    let greeting = use_state(|| None);

    let onclick = {
        let name = name.clone();
        let greeting = greeting.clone();

        Callback::from(move |_| {
            let name = name.clone();
            let greeting = greeting.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&format!("http://127.0.0.1:8000/api/greet/{}", *name))
                    .send()
                    .await
                    .unwrap()
                    .json::<GreetResponse>()
                    .await
                    .unwrap();
                greeting.set(Some(response.message));
            });
        })
    };

    html! {
        <div>
            <h1>{ "Machine Learning Image Classifier website" }</h1>
            <input
                type="text"
                value={(*name).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    name.set(input.value());
                })}
            />
            <button {onclick}>{ "Greet" }</button>
            <p>{ greeting.as_ref().unwrap_or(&"Waiting for response...".to_string()) }</p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}