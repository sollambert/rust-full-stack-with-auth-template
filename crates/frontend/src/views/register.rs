use yew::prelude::*;

use crate::components::auth::registerform::RegisterForm;

#[function_component(Register)]
pub fn login() -> Html {

    html! {
        <main class="flex flex-row grow content-center">
            <RegisterForm />
        </main>
    }
}