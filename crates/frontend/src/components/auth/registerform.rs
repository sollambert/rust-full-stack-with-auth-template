use types::user::RegisterUser;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};
use yew_hooks::use_async;
use gloo_console::error;
use yew_router::history::{History, HashHistory};
use yewdux::prelude::*;

use crate::{services, app::UserState, components::{buttons::button::Button, input::Input}};

#[function_component(RegisterForm)]
pub fn register_form() -> Html {
    let (_user_state, user_dispatch) = use_store::<UserState>();
    let register_user = use_state(RegisterUser::default);

    let oninput = |key| {
        let register_user = register_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match register_user.update_field(key, input.value()) {
                Ok(new_register_user) => {
                    register_user.set(new_register_user);
                }, Err(error) => {error!(error)}
            };
        })
    };

    let handle_register = {
        let register_user = register_user.clone();
        use_async(async move {
            let response = services::auth::register_user((*register_user).clone()).await;
            match response {
                Ok(user_info) => {
                    user_dispatch.set(UserState {user_info: user_info.clone()});
                    register_user.set(RegisterUser::default());
                    HashHistory::new().push("/login");
                    Ok(user_info)
                },
                Err(error) => {
                    Err(error)
                }
            }
        })
    };

    let register_onclick = {
        let handle_register = handle_register.clone();
        Callback::from(move |_| {
            handle_register.run();
        })
    };

    let register_onsubmit = {
        let handle_register = handle_register.clone();
        Callback::from(move |ev: SubmitEvent| {
            ev.prevent_default();
            handle_register.run();
        })
    };

    html! {
        <>
            <div class="m-4">
                <form class="flex flex-col w-64 h-64 space-y-2"
                    onsubmit={register_onsubmit}>
                    <Input input_type="text" placeholder="Username" oninput={oninput("username")} value={register_user.username.to_owned()} />
                    <Input input_type="password" placeholder="Password" oninput={oninput("pass")} value={register_user.pass.to_owned()} />
                    <Input input_type="email" placeholder="Email" oninput={oninput("email")} value={register_user.email.to_owned()} />
                    <Button onclick={register_onclick} label="Register" />
                </form>
            </div>
        </>
    }
}