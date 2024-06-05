use email_address::EmailAddress;
use serde::Deserialize;
use types::user::{ResetUser, UserInfo};
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast, UseStateHandle};
use yew_hooks::use_async;
use gloo_console::error;
use yew_router::{history::{HashHistory, History}, hooks::use_location};
use yewdux::functional::use_store;

use crate::{components::{buttons::button::Button, error_message::ErrorMessage, input::Input}, hooks::StoredUserInfo, services::{self, AuthError, AuthStorage}};

#[derive(Deserialize, Debug)]
struct QueryParams {
    email: EmailAddress,
    key: String
}

#[function_component(ResetForm)]
pub fn register_form() -> Html {
    let location = use_location().unwrap();
    let query_params = location.query::<QueryParams>().unwrap();
    let initial_user = ResetUser {
        email_address: query_params.email,
        pass: String::new()
    };
    let (_user_state, user_dispatch) = use_store::<StoredUserInfo>();
    let error_state = use_state(|| None::<AuthError>);
    let reset_user = use_state(|| initial_user.clone());

    let oninput = |key, error_state: &UseStateHandle<Option<AuthError>>| {
        let error_state = error_state.clone();
        let register_user = reset_user.clone();
        Callback::from(move |e: InputEvent| {
            let error_state = error_state.clone();
            if let Some(_) = *error_state {
                error_state.set(None);
            }
            let input: HtmlInputElement = e.target_unchecked_into();
            match register_user.update_field(key, input.value()) {
                Ok(new_register_user) => {
                    register_user.set(new_register_user);
                }, Err(error) => {error!(error)}
            };
        })
    };

    let handle_reset = {
        let reset_user = reset_user.clone();
        let error_state = error_state.clone();
        use_async(async move {
            let response = services::auth::reset_user((*reset_user).clone(), query_params.key).await;
            match response {
                Ok(status) => {
                    user_dispatch.set(StoredUserInfo {user_info: UserInfo::default()});
                    reset_user.set(initial_user);
                    AuthStorage::clear();
                    HashHistory::new().push("/login");
                    Ok(status)
                },
                Err(error) => {
                    error_state.set(Some(error.to_owned()));
                    Err(error)
                }
            }
        })
    };

    let reset_onclick = {
        let handle_reset = handle_reset.clone();
        Callback::from(move |_| {
            handle_reset.run();
        })
    };

    let reset_onsubmit = {
        let handle_reset = handle_reset.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            handle_reset.run();
        })
    };

    html! {
        <form class="flex flex-col w-64 space-y-2" onsubmit={reset_onsubmit}>
            if let Some(error) = (*error_state).to_owned() {
                <ErrorMessage message={error.body().message} />
            }
            <Input input_type="password" placeholder="Password" oninput={oninput("pass", &error_state)} value={reset_user.pass.to_owned()} />
            <Button onclick={reset_onclick} label="Confirm" />
        </form>
    }
}