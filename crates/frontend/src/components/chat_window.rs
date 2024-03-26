use tauri_sys::tauri::invoke;
use web_sys::{HtmlInputElement, WebSocket};
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{services::AuthStorage, graphics::icons::send_icon::SendIcon, components::{buttons::button::Button, input::Input}};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub class: String
}

#[function_component(ChatWindow)]
pub fn chat_windows(props: &Props) -> Html {
    let props = props.clone();
    let chat_disabled = use_state(|| true);
    let chat_message = use_state(|| String::new());

    let history = use_list(vec![]);

    // Manually connect to websocket with custom options.
    let ws = {

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

    let oninput = {
        let chat_message = chat_message.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            chat_message.set(input.value());
        })
    };

    let send_chat = {
        let ws = ws.clone();
        let chat_message = chat_message.clone();
        Callback::from(move |_| {
                if *chat_message == String::new() {
                    return;
                }
                ws.send(chat_message.to_string());
                chat_message.set(String::new());
        })
    };

    let send_chat_submit = {
        let ws = ws.clone();
        let chat_message = chat_message.clone();
        Callback::from(move |e: SubmitEvent| {
                e.prevent_default();
                ws.send(chat_message.to_string());
                chat_message.set(String::new());
        })
    };

    use_effect_once(move || {
        ws.open();
        move || {ws.close()}
    });

    html! {
        <div class={props.class}>
            <div class="h-full px-4 py-2 py-2 
            bg-slate-100 text-slate-800
            border-slate-300 dark:border-slate-700 border
            dark:bg-slate-900 dark:text-slate-100
            rounded-md ring-offset-background disabled:pointer-events-none
            overflow-y-auto text-wrap shadow-md">
                {
                    for history.current().iter().map(|message| {
                        html! {
                            <p>{ message }</p>
                        }
                    })
                }
            </div>
            <form class="flex flex-row h-12 w-full space-x-2" onsubmit={send_chat_submit}>
                <Input input_type="text" placeholder="Message..." oninput={oninput} value={(*chat_message).to_owned()} />
                <Button onclick={send_chat} icon={html!(<SendIcon class="fill-slate-600 dark:fill-white"/>)} disabled={*chat_disabled}></Button>
            </form>
        </div>
    }
}