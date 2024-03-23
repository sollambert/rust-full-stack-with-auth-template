use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VNode};
use yew_router::history::{History, HashHistory};
use yewdux::functional::use_store;

use crate::{app::UserState, services};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: ChildrenRenderer<VNode>,
}

#[function_component(ProtectedRoute)]
pub fn protected_route(props: &Props) -> Html {

    let (user_state, user_dispatch) = use_store::<UserState>();

    use_effect(move || {
        if user_state.user_info.uuid == String::new() {
            yew::platform::spawn_local(async move {
                let user_info = services::user::get_user_info().await;
                if user_info.uuid == String::new() {
                    HashHistory::new().push("/login");
                }
                user_dispatch.set(UserState {user_info});
            });
        }
    });

    html! {
        <>
            { props.children.clone() }
        </>
    }
}