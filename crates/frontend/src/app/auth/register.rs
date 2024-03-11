use gloo_console::{error, log};
use types::user::RegisterUser;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};
use yew_hooks::use_async;

use yewdux::prelude::*;

use crate::{services, app::UserState, components::{button::Button, input::Input}};

#[function_component(Register)]
pub fn register() -> Html {
    let (user_state, user_dispatch) = use_store::<UserState>();
    let register_user = use_state(RegisterUser::default);

    let oninput_username = {
        let register_user = register_user.clone();
        Callback::from(move |e: InputEvent| {
            let mut updated_user = (*register_user).clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            updated_user.username = input.value();
            register_user.set(updated_user);
        })
    };

    let oninput_pass = {
        let register_user = register_user.clone();
        Callback::from(move |e: InputEvent| {
            let mut updated_user = (*register_user).clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            updated_user.pass = input.value();
            register_user.set(updated_user);
        })
    };

    let oninput_email = {
        let register_user = register_user.clone();
        Callback::from(move |e: InputEvent| {
            let mut updated_user = (*register_user).clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            updated_user.email = input.value();
            register_user.set(updated_user);
        })
    };

    let handle_register = {
        let register_user = register_user.clone();
        use_async(async move {
            let response = services::auth::register_user((*register_user).clone()).await;
            match response {
                Ok(response_user) => {
                    // (response_user.clone());
                    user_dispatch.set(UserState {response_user: response_user.clone()});
                    register_user.set(RegisterUser::default());
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
                <pre>{"UserInfo: \n"}{user_state.response_user.to_string()}</pre>
                <br/>
                <pre>{"RegisterUser: \n"}{register_user.to_owned().to_string()}</pre>
                <br/>
                <form class="flex flex-col w-64 h-64 space-y-2"
                    onsubmit={register_onsubmit}>
                    <Input input_type="text" placeholder="Username" oninput={oninput_username} value={register_user.username.to_owned()} />
                    <Input input_type="password" placeholder="Password" oninput={oninput_pass} value={register_user.pass.to_owned()} />
                    <Input input_type="email" placeholder="Email" oninput={oninput_email} value={register_user.email.to_owned()} />
                    <Button onclick={register_onclick} label="Register" />
                </form>
            </div>
        </>
    }
}