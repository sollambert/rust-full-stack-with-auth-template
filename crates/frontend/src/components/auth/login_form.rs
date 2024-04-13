use gloo_console::error;
use types::user::LoginUser;
use web_sys::HtmlInputElement;
use yew::UseStateHandle;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};
use yew_hooks::use_async;

use yew_router::history::History;
use yew_router::history::HashHistory;
use yewdux::prelude::*;

use crate::components::error_message::ErrorMessage;
use crate::services::AuthError;
use crate::{services, app::UserState, components::{buttons::button::Button, input::Input}};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let (_user_state, user_dispatch) = use_store::<UserState>();
    let login_user = use_state(LoginUser::default);
    let error_state = use_state(|| None::<AuthError>);

    let oninput = |key, error_state: &UseStateHandle<Option<AuthError>>| {
        let error_state = (*error_state).clone();
        let login_user = login_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let error_state = error_state.clone();
            if let Some(_) = *error_state {
                error_state.set(None);
            }
            match login_user.update_field(key, input.value()) {
                Ok(new_login_user) => {
                    login_user.set(new_login_user);
                }, Err(error) => {error!(error)}
            };
        })
    };

    let handle_login = {
        let error_state = error_state.clone();
        let login_user = login_user.clone();
        use_async(async move {
            let response = services::auth::login_user((*login_user).clone()).await;
            match response {
                Ok(user_info) => {
                    user_dispatch.set(UserState {user_info: user_info.clone()});
                    login_user.set(LoginUser::default());
                    HashHistory::new().push("/");
                    Ok(user_info)
                },
                Err(error) => {
                    error_state.set(Some(error.to_owned()));
                    Err(error)
                }
            }
        })
    };

    let login_onclick = {
        let handle_login = handle_login.clone();
        Callback::from(move |_| {
            handle_login.run();
        })
    };

    let login_onsubmit = {
        let handle_login = handle_login.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            handle_login.run();
        })
    };

    html! {
        <form class="flex flex-col w-64 space-y-2" onsubmit={login_onsubmit}>
            if let Some(error) = (*error_state).to_owned() {
                <ErrorMessage message={error.body().message} />
            }
            <Input input_type="text" placeholder="Username" oninput={oninput.clone()("username", &error_state)} value={login_user.username.to_owned()} />
            <Input input_type="password" placeholder="Password" oninput={oninput.clone()("pass", &error_state)} value={login_user.pass.to_owned()} />
            <Button onclick={login_onclick} label="Login" />
        </form>
    }
}