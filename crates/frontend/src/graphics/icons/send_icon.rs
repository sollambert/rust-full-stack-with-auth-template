use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub class: String
}

#[function_component(SendIcon)]
pub fn send_icon(props: &Props) -> Html {
    let props = props.clone();

    html!(
        <svg class={props.class} xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
            <path d="M120-160v-640l760 320-760 320Zm80-120 474-200-474-200v140l240 60-240 60v140Zm0 0v-400 400Z"/>
        </svg>
    )

}