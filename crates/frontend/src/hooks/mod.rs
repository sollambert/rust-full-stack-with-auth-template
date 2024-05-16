use yew::prelude::*;
use yewdux::prelude::use_store;
use crate::{app::UserState, services};
use types::user::UserInfo;


#[hook]
pub fn use_user_info() -> UserInfo {
    let (user_state, user_dispatch) = use_store::<UserState>();

    let user_state_clone = user_state.clone();
    use_effect(move || {
        if user_state_clone.user_info.uuid == String::new() {
            yew::platform::spawn_local(async move {
                let user_info = services::user::get_user_info().await;
                user_dispatch.set(UserState {user_info});
            });
        }
    });

    return user_state.user_info.clone();
}