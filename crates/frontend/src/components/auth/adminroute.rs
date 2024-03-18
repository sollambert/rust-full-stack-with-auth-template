use yew::prelude::*;
use yew_router::history::{History, HashHistory};
use yewdux::functional::use_store;

use crate::app::UserState;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(AdminRoute)]
pub fn admin_route(props: &Props) -> Html {

    let (user_state, _user_dispatch) = use_store::<UserState>();

    use_effect(move || {
        if user_state.user_info.uuid == String::new() || !user_state.user_info.is_admin {
            HashHistory::new().push("/login");
        }
    });

    html! {
        <>
            { props.children.clone() }
        </>
    }
}