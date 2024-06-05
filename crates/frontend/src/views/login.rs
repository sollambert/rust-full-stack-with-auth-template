use yew::prelude::*;
use yew_router::history::History;
use yew_router::history::HashHistory;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::components::auth::login_form::LoginForm;
use crate::hooks::use_user_info;

#[function_component(Login)]
pub fn login() -> Html {
    let user_info = use_user_info();

    use_effect(move || {
        if user_info.uuid != String::new() {
            HashHistory::new().push("/")
        }
    });

    html! {
        <div class="col-span-12 row-span-24 flex flex-col justify-center items-center h-full space-y-4">
            <LoginForm />
            <Link<AppRoute> to={AppRoute::Register}>
                <div class="cursor-pointer text-blue-600 dark:text-blue-400 underline
                        focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-3">
                    {"Need an account?"}
                </div>
            </Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::RequestReset}>
                <div class="cursor-pointer text-blue-600 dark:text-blue-400 underline
                        focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-3">
                    {"Forgot password?"}
                </div>
            </Link<AppRoute>>
        </div>
    }
}