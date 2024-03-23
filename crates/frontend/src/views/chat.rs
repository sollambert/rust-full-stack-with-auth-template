use yew::prelude::*;
use crate::components::chat_window::ChatWindow;

#[function_component(Chat)]
pub fn chat() -> Html {
    html! {
        <main class="h-full">
            <ChatWindow class="flex shrink flex-col w-full h-full
            ring-offset-background disabled:pointer-events-none
            p-4 bg-slate-700 text-slate-100 space-y-2 text-sm"/>
        </main>
    }
}