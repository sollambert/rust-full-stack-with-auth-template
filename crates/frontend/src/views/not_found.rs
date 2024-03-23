use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {

    html! {
        <div class="flex flex-col justify-center items-center h-full space-y-4">
            <p class="text-8xl text-slate-200 tracking-[.15em] pb-4 rotate-90">{":("}</p>
            <p class="text-2xl text-slate-200">{"Page not found"}</p>
        </div>
    }
}