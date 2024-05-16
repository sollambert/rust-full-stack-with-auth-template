use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VNode};
use yew_router::history::{History, HashHistory};

use crate::hooks::use_user_info;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: ChildrenRenderer<VNode>,
}

#[function_component(ProtectedRoute)]
pub fn protected_route(props: &Props) -> Html {

    let user_info = use_user_info();

    use_effect(move || {
        if user_info.uuid == String::new() {
            HashHistory::new().push("/login");
        }
    });

    html! {
        <>
            { props.children.clone() }
        </>
    }
}