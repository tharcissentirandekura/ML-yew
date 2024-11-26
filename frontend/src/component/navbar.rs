use crate::component::button::{Button, FileInput};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub onclick: Callback<SubmitEvent>,
    pub on_file_change: Callback<Event>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
        <main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
            <div class="text-center">
                <h1 class="mt-4 text-balance text-5xl font-semibold tracking-tight text-red-600 sm:text-7xl">{"ClassiRust"}</h1>
                <p class="mt-6 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">{"Classify your Image with confidence using Machine Learning."}</p>
                <form class="p-10 bg-sky-200" onsubmit={props.onclick.clone()}>
                    <label class="flex flex-col items-center py-7 text-blue rounded-lg shadow-lg tracking-wide uppercase border cursor-pointer border-red bg-gray-100 hover:bg-indigo-500 hover:text-white">
                        <svg class="w-8 h-8" fill="blue" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                            <path d="M16.88 9.94l-4.88-4.88v3.94h-4v-3.94l-4.88 4.88 1.41 1.41 3.47-3.47v3.94h4v-3.94l3.47 3.47 1.41-1.41z"/>
                        </svg>
                        <span class="mt-2 text-2xl font-semibold leading-normal">{"Select a file"}</span>
                        <FileInput
                            class="hidden"
                            onchange={props.on_file_change.clone()}
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
