use types::user::ResponseUser;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(ResponseUser::default);

    html! {
        <ContextProvider<UseStateHandle<ResponseUser>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<ResponseUser>>>
    }
}