use gloo_console::error;
use types::user::LoginUser;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};
use yew_hooks::use_async;

use yew_router::history::History;
use yew_router::history::HashHistory;
use yewdux::prelude::*;

use crate::{services, app::UserState, components::{buttons::button::Button, input::Input}};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let (_user_state, user_dispatch) = use_store::<UserState>();
    let login_user = use_state(LoginUser::default);
    let error_message = use_state(|| String::new());

    let oninput = |key| {
        let login_user = login_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match login_user.update_field(key, input.value()) {
                Ok(new_login_user) => {
                    login_user.set(new_login_user);
                }, Err(error) => {error!(error)}
            };
        })
    };

    let handle_login = {
        let error_message = error_message.clone();
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
                    error_message.set(String::from("Invalid login credentials"));
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
        <form class="flex flex-col w-64 h-48 space-y-2" onsubmit={login_onsubmit}>
            if *error_message != String::new() {
                <div class="px-4 py-2 rounded-md bg-red-300 text-red-600 text-center shadow-md">
                    {(*error_message).clone()}
                </div>
            }
            <Input input_type="text" placeholder="Username" oninput={oninput.clone()("username")} value={login_user.username.to_owned()} />
            <Input input_type="password" placeholder="Password" oninput={oninput.clone()("pass")} value={login_user.pass.to_owned()} />
            <Button onclick={login_onclick} label="Login" />
        </form>
    }
}