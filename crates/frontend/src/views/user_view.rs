use types::user::UserInfo;
use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::functional::use_store;

use crate::{components::{buttons::button::Button, error_message::ErrorMessage, user_info_panel::UserInfoPanel}, services::{self, AuthError}};
use crate::hooks::StoredUserInfo;

#[function_component(UserView)]
pub fn user_view() -> Html {
    let (_user_info, user_info_dispatch) = use_store::<StoredUserInfo>();
    let error_state = use_state(|| None::<AuthError>);

    let logout_onclick = {
        Callback::from(move |_| {
            services::auth::logout_user();
            user_info_dispatch.set(StoredUserInfo { user_info: UserInfo::default() });
        })
    };

    let handle_test = {
        let error_state = error_state.clone();
        use_async(async move {
            let response = services::auth::test_auth_route().await;
            match response {
                Ok(status_code) => {
                    Ok(status_code)
                },
                Err(error) => {
                    error_state.set(Some(error));
                    Err(())
                }
            }
        })
    };

    let test_onclick = {
        let handle_test = handle_test.clone();
        Callback::from(move |_| {
            handle_test.run();
        })
    };

    html! {
        <div class="col-span-12 row-span-24 flex flex-col justify-center items-center h-full space-y-2">
            if let Some(error) = (*error_state).to_owned() {
                <ErrorMessage message={error.body().message} />
            }
            <UserInfoPanel />
            <div class="flex flex-row space-x-4">
                <Button label={"Logout"} onclick={logout_onclick} />
                <Button onclick={test_onclick} label={"Test Auth"} />
            </div>
        </div>
    }
}