use yew::prelude::*;
use yewdux::prelude::{Store, use_store};
use types::user::UserInfo;
use crate::services;

#[derive(Default, PartialEq, Store)]
pub struct StoredUserInfo {
    pub user_info: UserInfo
}

#[hook]
pub fn use_user_info() -> UserInfo {
    let (stored_user_info, user_info_dispatch) = use_store::<StoredUserInfo>();
    let user_info = stored_user_info.user_info.clone();

    let user_info_clone = user_info.clone();
    use_effect(move || {
        if user_info_clone.uuid == String::new() {
            yew::platform::spawn_local(async move {
                let user_info = services::user::get_user_info().await;
                user_info_dispatch.set(StoredUserInfo{user_info});
            });
        }
    });

    return user_info.clone();
}