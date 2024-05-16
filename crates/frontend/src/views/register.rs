use yew::prelude::*;
use yew_router::history::History;
use yew_router::history::HashHistory;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::components::auth::register_form::RegisterForm;
use crate::hooks::use_user_info;

#[function_component(Register)]
pub fn login() -> Html {
    let user_info = use_user_info();

    use_effect(move || {
        if user_info.uuid != String::new() {
            HashHistory::new().push("/")
        }
    });

    html! {
        <div class="col-span-12 row-span-24 flex flex-col justify-center items-center h-full space-y-4">
            <RegisterForm />
            <Link<AppRoute> to={AppRoute::Login}>
                <div class="cursor-pointer text-blue-600 dark:text-blue-400 underline
                        focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-3">
                    {"Already have an account?"}
                </div>
            </Link<AppRoute>>
        </div>
    }
}