use yew::prelude::*;
use crate::component::button::Button;


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
}
#[function_component(Navbar)]
pub fn navbar(props:&ButtonProps) -> Html {
    html! {
        <main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
        <div class="text-center">

            <h1 class="mt-4 text-balance text-5xl font-semibold tracking-tight text-red-600 sm:text-7xl">{"ClassiRust"}</h1>
            <p class="mt-6 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">{"Indentify what is in your image."}</p>
            <div class="mt-10 flex items-center justify-center gap-x-6">
            <Button on_click={props.onclick.clone()} class="rounded-md bg-red-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600" content="Get started"/>
            <a href="/try" class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{"Try it yourself"}</a>
            <a href="#" class="text-sm font-semibold text-gray-900">{"Send feedback.. "}<span aria-hidden="true"></span></a>
          </div>
        </div>
      </main>
    }
}