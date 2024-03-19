use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {

    html! {
        <main class="flex flex-col items-center h-100">
            <p class="text-xl">{"Page not found"}</p>
        </main>
    }
}