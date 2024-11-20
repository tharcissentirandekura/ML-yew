use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub on_click: Callback<MouseEvent>,
    pub class: String,
    pub content: String,
}

#[function_component(Button)]
pub fn button(props:&ButtonProps) -> Html {
    html! {
        <button onclick={props.on_click.clone()} class={props.class.clone()}>
            {props.content.clone()}
        </button>
    }
}