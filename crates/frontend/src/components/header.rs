use yew::prelude::*;
use crate::{app::AppRoute, components::buttons::nav_button::NavButton, hooks::use_user_info};

#[function_component(Header)]
pub fn header() -> Html {
    let user_info = use_user_info();

    html! {
        <header class="flex flex-row h-12 z-10 fixed border-slate-300 dark:border-slate-700 border-b
            bg-slate-100 dark:bg-slate-900 shadow-md w-screen
            justify-between justify-items-center items-center">
            <div class="flex flex-row h-full">
                <NavButton label="Home" destination={AppRoute::Home} />
                if user_info.uuid != String::new() {
                    <NavButton label="Chat" destination={AppRoute::Chat} />
                }
            </div>
            <div class="flex flex-row h-full">
                if user_info.uuid != String::new() {
                    if user_info.is_admin {
                        <NavButton label="Admin" destination={AppRoute::AdminPanel} />
                    }
                    <NavButton label={user_info.clone().username} destination={AppRoute::UserPanel} />
                } else {
                        <NavButton label="Login" destination={AppRoute::Login} />
                        <NavButton label="Register" destination={AppRoute::Register} />
                }
            </div>
        </header>
    }
}