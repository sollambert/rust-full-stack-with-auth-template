use yew::prelude::*;
use yew_router::history::History;
use yew_router::history::HashHistory;

use crate::components::auth::registerform::RegisterForm;

#[function_component(Register)]
pub fn login() -> Html {

    html! {
        <main class="flex flex-col items-center h-100">
            <RegisterForm />
            <a class="cursor-pointer text-blue-700 underline" onclick={move |_| {HashHistory::new().push("/login")}}>{"Already have an account?"}</a>
        </main>
    }
}