use gloo_console::error;
use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::{components::buttons::button::Button, services};

#[function_component(Home)]
pub fn home() -> Html {

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
        <div class="flex flex-row justify-center items-center h-full">
            <p class="space-x-4 m-4">
                <Button onclick={test_onclick} label={"Test Auth"} />
            </p>
        </div>
    }
}