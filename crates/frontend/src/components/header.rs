use yew::prelude::*;
use yewdux::functional::use_store;
use crate::{app::{AppRoute, UserState}, components::buttons::navbutton::NavButton};

#[function_component(Header)]
pub fn header() -> Html {
    let (user_state, _user_dispatch) = use_store::<UserState>();

    html! {
        <header class="flex flex-row bg-slate-900 justify-between justify-items-center">
            <NavButton label="Home" destination={AppRoute::Home} />
            if user_state.user_info.uuid != String::new() {
                if user_state.user_info.is_admin {
                    <NavButton label="Admin" destination={AppRoute::Login} />
                }
                <NavButton label={user_state.user_info.clone().username} destination={AppRoute::Login} />
            } else {
                <div class="flex flex-row">
                    <NavButton label="Login" destination={AppRoute::Login} />
                    <NavButton label="Register" destination={AppRoute::Register} />
                </div>
            }
        </header>
    }
}