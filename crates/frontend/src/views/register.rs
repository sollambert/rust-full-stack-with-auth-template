use yew::prelude::*;
use yew_router::history::History;
use yew_router::history::HashHistory;
use yewdux::functional::use_store;

use crate::app::UserState;
use crate::components::auth::register_form::RegisterForm;

#[function_component(Register)]
pub fn login() -> Html {
    let (user_state, _user_dispatch) = use_store::<UserState>();

    use_effect(move || {
        if user_state.user_info.uuid != String::new() {
            HashHistory::new().push("/")
        }
    });

    html! {
        <main class="flex flex-col items-center h-100">
            <RegisterForm />
            <div tabindex={0} class="cursor-pointer text-blue-700 underline
                    focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-3"
                    onclick={move |_| {HashHistory::new().push("/login")}}
                    onkeypress={move |e: KeyboardEvent| {if e.key() == "Enter" { HashHistory::new().push("/login")}}}>
                {"Already have an account?"}
            </div>
        </main>
    }
}