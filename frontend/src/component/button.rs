use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: String,
    #[prop_or("Click me".to_string())]
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub placeholder: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button onclick={props.onclick.clone()} class={props.class.clone()}>
            {props.label.clone()}
        </button>
    }
}

#[function_component(FileInput)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <input
            type="file"
            class={props.class.clone()}
            placeholder={props.placeholder.clone()}
            onchange={props.onchange.clone()}
            accept="image/*"
        />
    }
}
