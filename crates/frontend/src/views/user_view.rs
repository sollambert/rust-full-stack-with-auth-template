use yew::prelude::*;

use crate::components::user_info_panel::UserInfoPanel;

#[function_component(UserView)]
pub fn user_view() -> Html {

    html! {
        <main class="flex flex-col items-center h-100">
            <UserInfoPanel />
        </main>
    }
}