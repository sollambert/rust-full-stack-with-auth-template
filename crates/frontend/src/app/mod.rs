pub mod auth;

use gloo_net::http::Request;
use tauri_sys::tauri::invoke;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_hooks::prelude::*;
// use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};

use types::user::UserInfo;
use crate::{app::auth::registerform::RegisterForm, components::button::Button};


#[derive(Default, PartialEq, Store)]
struct UserState {
    user_info: UserInfo
}


#[function_component(App)]
pub fn app() -> Html {
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
        use_websocket_with_options(
            format!("ws://localhost:{}/ws", port),
            UseWebSocketOptions {
                // Receive message by callback `onmessage`.
                onmessage: Some(Box::new(move |message| {
                    history.push(format!("ws [recv]: {}", message));
                })),
                manual: Some(true),
                ..Default::default()
            },
        )
    };
    let onclick2 = {
        let ws = ws.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, backend!".to_string();
            ws.send(message.clone());
            history.push(format!("ws [send]: {}", message));
        })
    };
    let onopen = {
        let ws = ws.clone();
        Callback::from(move |_| {
            ws.open();
        })
    };
    html! {
        <>
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
                <Button onclick={onopen} label={"Connect to backend websocket"} disabled={*ws.ready_state != UseWebSocketReadyState::Closed} />
                <Button onclick={onclick2} label={"Send to backend websocket"} disabled={*ws.ready_state != UseWebSocketReadyState::Open} />
            </p>
            {
                for history.current().iter().map(|message| {
                    html! {
                        <p>{ message }</p>
                    }
                })
            }
            <RegisterForm />
        </>
    }
}
