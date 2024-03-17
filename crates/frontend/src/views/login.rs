use yew::prelude::*;
use yew_router::history::History;
use yew_router::history::HashHistory;

use crate::components::auth::loginform::LoginForm;

#[function_component(Login)]
pub fn login() -> Html {

    html! {
        <main class="flex flex-col items-center h-100">
            <LoginForm />
            <a class="cursor-pointer text-blue-700 underline" onclick={move |_| {HashHistory::new().push("/register")}}>{"Need an account?"}</a>
        </main>
    }
}