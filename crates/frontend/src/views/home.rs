use gloo_console::error;
use gloo_net::http::Request;
use tauri_sys::tauri::invoke;
use web_sys::WebSocket;
use yew::prelude::*;
use yew_hooks::prelude::*;

use types::user::UserInfo;
use crate::{components::{buttons::button::Button, user_info_panel::UserInfoPanel}, services::{self, AuthStorage}};

#[function_component(Home)]
pub fn home() -> Html {

    // Get backend port automatically from tauri command.
    let port = use_async_with_options(
        async move {
            match invoke::<_, String>("get_port", &()).await {
                Ok(p) => Ok(p),
                Err(e) => Err(format!("Error: {:?}", e)),
            }
        },
        UseAsyncOptions::enable_auto(),
    );

    let chat_disabled = use_state(|| true);

    // Fetch data from backend.
    let state = {
        let port = port.clone();
        use_async(async move {
            match &port.data {
                Some(port) => {
                    let response = Request::get(format!("http://localhost:{}/user", port).as_str()).send().await;
                    match response {
                        Ok(data) => match data.json::<UserInfo>().await {
                            Ok(user) => Ok(user),
                            Err(_) => Err("Backend body Error".to_owned()),
                        },
                        Err(_) => Err("Backend request Error".to_owned()),
                    }
                }
                _ => Err("Backend is unavailable".to_owned()),
            }
        })
    };

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    // Fetch data from server.
    let state_server = use_async(async move {
        let response = Request::get("http://localhost:3001/user").send().await;
        match response {
            Ok(data) => match data.json::<UserInfo>().await {
                Ok(user) => Ok(user),
                Err(_) => Err("Body Error".to_string()),
            },
            Err(_) => Err("Request Error".to_string()),
        }
    });

    let onclickserver = {
        let state_server = state_server.clone();
        Callback::from(move |_| {
            state_server.run();
        })
    };

    let history = use_list(vec![]);

    // Manually connect to websocket with custom options.
    let ws = {
        let history = history.clone();
        let mut port = port.data.clone().unwrap_or_default();
        if cfg!(debug_assertions) && port == "" {
            port = "3001".to_string();
        }
        let chat_disabled_for_open = chat_disabled.clone();
        let chat_disabled_for_close = chat_disabled.clone();
        use_websocket_with_options(
            format!("ws://localhost:{}/ws", port),
            UseWebSocketOptions {
                onopen: Some(Box::new(move |event| {
                    let socket = event.target_dyn_into::<WebSocket>().unwrap();
                    if let Ok(token) = AuthStorage::get_requester_token() {
                        socket.send_with_str(&token.access_token).unwrap();
                        chat_disabled_for_open.set(false);
                    } else {
                        socket.close().unwrap();
                    }
                })),
                // Receive message by callback `onmessage`.
                onmessage: Some(Box::new(move |message| {
                    history.push(format!("{}", message));
                })),
                onclose: Some(Box::new(move |_event| {
                    chat_disabled_for_close.set(true);
                })),
                manual: Some(true),
                ..Default::default()
            },
        )
    };

    let onclick2 = {
        let ws = ws.clone();
        let message = "Hello, backend!";
        Callback::from(move |_| {
                ws.send(message.to_string());
        })
    };

    use_effect_once(move || {
        ws.open();
        move || {ws.close()}
    });

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
        <main>
            <p class="space-x-4 m-4">
                <Button onclick={onclick} label={"Load backend api"} />
                <Button onclick={onclickserver} label={"Load server api"} />
            </p>
            {
                if let Some(response) = &state.data {
                    html! {
                        <p>{ "From backend: " }<b>{ &response.username }</b></p>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(response) = &state_server.data {
                    html! {
                        <p>{ "From server: " }<b>{ &response.username }</b></p>
                    }
                } else {
                    html! {}
                }
            }
            <p class="space-x-4 m-4">
                <Button onclick={onclick2} label={"Send chat"} disabled={*chat_disabled} />
            </p>
            <p class="space-x-4 m-4">
                <Button onclick={test_onclick} label={"Test Auth"} />
            </p>
            {
                for history.current().iter().map(|message| {
                    html! {
                        <p>{ message }</p>
                    }
                })
            }
            <p class="space-x-4 m-4">
                <UserInfoPanel />
            </p>
        </main>
    }
}