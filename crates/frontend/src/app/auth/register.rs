use gloo_console::error;
use types::user::RegisterUser;
use yew::{function_component, html, use_state, Callback, Html};
use yew_hooks::use_async;

use yewdux::prelude::*;

use crate::{services, UserState};

#[function_component(Register)]
pub fn register() -> Html {
    let (user_state, user_dispatch) = use_store::<UserState>();
    let username = use_state(|| "test2");
    let pass = use_state(|| "test2");
    let email = use_state(|| "test2");

    let handle_register = use_async(async move {
        let response = services::auth::register_user(
            RegisterUser {
                username: username.to_string(),
                pass: pass.to_string(),
                email: email.to_string()
            }).await;
        match response {
            Ok(response_user) => {
                // (response_user.clone());
                user_dispatch.set(UserState {response_user: response_user.clone()});
                Ok(response_user)
            },
            Err(error) => {
                error!("No response found: {}", error.to_string());
                Err(error)
            }
        }
    });

    let register_onclick = {
        let handle_register = handle_register.clone();
        Callback::from(move |_| {
            handle_register.run();
        })
    };

    html! {
        <>
            <pre>{user_state.response_user.to_owned()}</pre>
            <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90" onclick={register_onclick}>{"Register"}</button>
        </>
    }
}