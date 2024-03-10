use gloo_console::error;
use types::user::CreateUser;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{hooks::use_user_context::use_user_context, services};

#[function_component(Register)]
pub fn register() -> Html {
    let username = use_state(|| "");
    let pass = use_state(|| "");
    let email = use_state(|| "");

    let user_cxt = use_user_context();

    let register_onclick = use_async(async move {
        let response = services::auth::register_user(
            CreateUser {
                username: username.to_string(),
                pass: pass.to_string(),
                email: email.to_string()
            }).await;
        match response {
            Ok(response_user) => {
                user_cxt.set_user(response_user.clone());
                Ok(response_user)
            },
            Err(error) => {
                error!("No response found: {}", error.to_string());
                Err(error)
            }
        }
    });

    html! {
        <>
        </>
    }
}