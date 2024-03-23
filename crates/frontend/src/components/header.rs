use yew::prelude::*;
use yewdux::functional::use_store;
use crate::{app::{AppRoute, UserState}, components::buttons::navbutton::NavButton};

#[function_component(Header)]
pub fn header() -> Html {
    let (user_state, _user_dispatch) = use_store::<UserState>();

    html! {
        <header class="col-span-12 row-span-1 flex flex-row
            bg-slate-100 dark:bg-slate-900 shadow-md
            justify-between justify-items-center items-center">
            <div class="flex flex-row h-full">
                <NavButton label="Home" destination={AppRoute::Home} />
                if user_state.user_info.uuid != String::new() {
                    <NavButton label="Chat" destination={AppRoute::Chat} />
                }
            </div>
            <div class="flex flex-row h-full">
                if user_state.user_info.uuid != String::new() {
                    if user_state.user_info.is_admin {
                        <NavButton label="Admin" destination={AppRoute::AdminPanel} />
                    }
                    <NavButton label={user_state.user_info.clone().username} destination={AppRoute::UserPanel} />
                } else {
                        <NavButton label="Login" destination={AppRoute::Login} />
                        <NavButton label="Register" destination={AppRoute::Register} />
                }
            </div>
        </header>
    }
}