use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::functional::use_store;
use gloo_console::error;

use crate::{app::UserState, components::{buttons::button::Button, user_info_panel::UserInfoPanel}, services};

#[function_component(UserView)]
pub fn user_view() -> Html {
    let (_user_state, user_dispatch) = use_store::<UserState>();

    let logout_onclick = {
        Callback::from(move |_| {
            services::auth::logout_user();
            user_dispatch.set(UserState::default());
        })
    };

    let handle_test = {
        use_async(async move {
            let response = services::auth::test_auth_route().await;
            match response {
                Ok(status_code) => {
                    Ok(status_code)
                },
                Err(error) => {
                    error!("No response found: {}", error.to_string());
                    Err(error)
                }
            }
        })
    };

    let test_onclick = {
        let handle_test = handle_test.clone();
        Callback::from(move |_| {
            handle_test.run();
        })
    };

    html! {
        <div class="flex flex-col justify-center items-center h-full space-y-2">
            <UserInfoPanel />
            <div class="flex flex-row space-x-4">
                <Button label={"Logout"} onclick={logout_onclick} />
                <Button onclick={test_onclick} label={"Test Auth"} />
            </div>
        </div>
    }
}