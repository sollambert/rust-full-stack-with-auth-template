use yew::prelude::*;

use crate::hooks::use_user_info;

#[function_component(UserInfoPanel)]
pub fn user_info_panel() -> Html {

    let user_info = use_user_info();

    html! {
        <div class="flex flex-col justify-center w-fit h-min
        rounded-md text-lg font-strong ring-offset-background disabled:pointer-events-none
        focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2
        h-10 px-4 py-2
        border-slate-300 dark:border-slate-700 border
        bg-slate-100 text-slate-800 shadow-md
        dark:bg-slate-900 dark:text-slate-100">
            <p>
                {format!("UUID: {}", user_info.uuid.clone())}
            </p>
            <p>
                {format!("Username: {}", user_info.username.clone())}
            </p>
            <p>
                {format!("Email: {}", user_info.email.clone())}
            </p>
            <p>
                {format!("Is Admin: {}", user_info.is_admin.clone())}
            </p>
        </div>
    }
}