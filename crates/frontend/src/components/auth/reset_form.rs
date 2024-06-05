use email_address::EmailAddress;
use serde::Deserialize;
use types::{auth::AuthErrorType, user::{ResetUser, UserInfo}};
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
pub fn reset_form() -> Html {
    let location = use_location().unwrap();
    let query_params = location.query::<QueryParams>().unwrap();
    let initial_user = ResetUser {
        email_address: query_params.email,
        pass: String::new()
    };
    let (_user_state, user_dispatch) = use_store::<StoredUserInfo>();
    let error_state = use_state(|| None::<AuthError>);
    let reset_user = use_state(|| initial_user.clone());
    let confirm_pass = use_state(|| String::new());

    let oninput = |key, error_state: &UseStateHandle<Option<AuthError>>| {
        let error_state = error_state.clone();
        let reset_user = reset_user.clone();
        Callback::from(move |e: InputEvent| {
            let error_state = error_state.clone();
            if let Some(_) = *error_state {
                error_state.set(None);
            }
            let input: HtmlInputElement = e.target_unchecked_into();
            match reset_user.update_field(key, input.value()) {
                Ok(new_reset_user) => {
                    reset_user.set(new_reset_user);
                }, Err(error) => {error!(error)}
            };
        })
    };

    let on_confirm_input = |error_state: &UseStateHandle<Option<AuthError>>| {
        let error_state = error_state.clone();
        let reset_user = reset_user.clone();
        let confirm_pass = confirm_pass.clone();
        Callback::from(move |e: InputEvent| {
            let confirm_pass_value = e.target_unchecked_into::<HtmlInputElement>().value();
            if confirm_pass_value != reset_user.pass {
                error_state.set(Some(AuthError::from_error_type(AuthErrorType::PasswordDoesNotMatch)));
            } else {
                error_state.set(None);
            }
            confirm_pass.set(confirm_pass_value);
        }) 
    };

    let handle_reset = {
        let reset_user = reset_user.clone();
        let error_state = error_state.clone();
        use_async(async move {
            let response = services::auth::reset_user((*reset_user).clone(), query_params.key).await;
            if let Some(error) = (*error_state).to_owned() {
                match error.body().error_type {
                    AuthErrorType::PasswordDoesNotMatch => return Err(error),
                    _ => {()}
                }
            }
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
        <form class="flex flex-col w-64 space-y-2 text-center
            text-slate-800 dark:text-slate-100" onsubmit={reset_onsubmit}>
            <p>{"Enter your new password"}</p>
            if let Some(error) = (*error_state).to_owned() {
                <ErrorMessage message={error.body().message} />
            }
            <Input input_type="password" placeholder="Password" oninput={oninput("pass", &error_state)} value={reset_user.pass.to_owned()} />
            <Input input_type="password" placeholder="Confirm password" oninput={on_confirm_input(&error_state)} value={(*confirm_pass).to_owned()} />
            <Button onclick={reset_onclick} label="Confirm" />
        </form>
    }
}