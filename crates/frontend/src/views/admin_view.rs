use yew::prelude::*;

#[function_component(AdminView)]
pub fn admin_view() -> Html {

    html! {
        <main class="flex flex-col items-center h-100">
            <p>{"This is the admin panel"}</p>
        </main>
    }
}