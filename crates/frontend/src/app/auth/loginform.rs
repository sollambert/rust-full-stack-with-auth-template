use gloo_console::{error, log};
use types::user::LoginUser;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};
use yew_hooks::use_async;

use yewdux::prelude::*;

use crate::{services, app::UserState, components::{button::Button, input::Input}};

#[function_component(RegisterForm)]
pub fn register_form() -> Html {
    let (user_state, user_dispatch) = use_store::<UserState>();
    let login_user = use_state(LoginUser::default);

    let oninput_username = {
        let login_user = login_user.clone();
        Callback::from(move |e: InputEvent| {
            let mut updated_user = (*login_user).clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            updated_user.username = input.value();
            login_user.set(updated_user);
        })
    };

    let oninput_pass = {
        let login_user = login_user.clone();
        Callback::from(move |e: InputEvent| {
            let mut updated_user = (*login_user).clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            updated_user.pass = input.value();
            login_user.set(updated_user);
        })
    };

    let handle_login = {
        let login_user = login_user.clone();
        use_async(async move {
            let response = services::auth::login_user((*login_user).clone()).await;
            match response {
                Ok(response_user) => {
                    // (response_user.clone());
                    user_dispatch.set(UserState {response_user: response_user.clone()});
                    login_user.set(LoginUser::default());
                    Ok(response_user)
                },
                Err(error) => {
                    error!("No response found: {}", error.to_string());
                    Err(error)
                }
            }
        })
    };

    let register_onclick = {
        let handle_login = handle_login.clone();
        Callback::from(move |_| {
            handle_login.run();
        })
    };

    let register_onsubmit = {
        let handle_login = handle_login.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            handle_login.run();
        })
    };

    html! {
        <>
            <div class="m-4">
                <pre>{"UserInfo: \n"}{user_state.response_user.to_string()}</pre>
                <br/>
                <form class="flex flex-col w-64 h-64 space-y-2"
                    onsubmit={register_onsubmit}>
                    <Input input_type="text" placeholder="Username" oninput={oninput_username} value={login_user.username.to_owned()} />
                    <Input input_type="password" placeholder="Password" oninput={oninput_pass} value={login_user.pass.to_owned()} />
                    <Button onclick={register_onclick} label="Register" />
                </form>
            </div>
        </>
    }
}