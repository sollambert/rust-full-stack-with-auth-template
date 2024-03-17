use yew::prelude::*;

use crate::components::auth::loginform::LoginForm;

#[function_component(Login)]
pub fn login() -> Html {

    html! {
        <main class="flex flex-col items-center h-100">
            <LoginForm />
        </main>
    }
}