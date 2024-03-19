use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VNode};
use yew_router::history::{History, HashHistory};
use yewdux::functional::use_store;

use crate::app::UserState;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: ChildrenRenderer<VNode>,
}

#[function_component(ProtectedRoute)]
pub fn protected_route(props: &Props) -> Html {

    let (user_state, _user_dispatch) = use_store::<UserState>();

    use_effect(move || {
        if user_state.user_info.uuid == String::new() {
            HashHistory::new().push("/login");
        }
    });

    html! {
        <>
            { props.children.clone() }
        </>
    }
}