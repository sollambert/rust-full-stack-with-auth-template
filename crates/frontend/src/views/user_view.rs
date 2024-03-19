use yew::prelude::*;
use yewdux::functional::use_store;

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

    html! {
        <main class="flex flex-col items-center h-100">
            <UserInfoPanel />
            <Button label={"Logout"} onclick={logout_onclick} />
        </main>
    }
}