use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast, UseStateHandle};
use yew_hooks::use_async;
use yew_router::history::{HashHistory, History};

use crate::{components::{buttons::button::Button, error_message::ErrorMessage, input::Input}, services::{self, AuthError, AuthStorage}};

#[function_component(RequestResetForm)]
pub fn request_reset_form() -> Html {
    let error_state = use_state(|| None::<AuthError>);
    let reset_email = use_state(|| String::new());

    let oninput = |error_state: &UseStateHandle<Option<AuthError>>| {
        let error_state = error_state.clone();
        let reset_email = reset_email.clone();
        Callback::from(move |e: InputEvent| {
            let error_state = error_state.clone();
            if let Some(_) = *error_state {
                error_state.set(None);
            }
            let input: HtmlInputElement = e.target_unchecked_into();
            reset_email.set(input.value());
        })
    };

    let handle_reset = {
        let reset_email = reset_email.clone();
        let error_state = error_state.clone();
        use_async(async move {
            let response = services::auth::request_reset((*reset_email).to_owned()).await;
            match response {
                Ok(status) => {
                    reset_email.set(String::new());
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
            <p>{"Enter your email to reset your password"}</p>
            if let Some(error) = (*error_state).to_owned() {
                <ErrorMessage message={error.body().message} />
            }
            <Input input_type="email" placeholder="Email" oninput={oninput(&error_state)} value={(*reset_email).to_owned()} />
            <Button onclick={reset_onclick} label="Confirm" />
        </form>
    }
}