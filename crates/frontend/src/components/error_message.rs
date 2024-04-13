use yew::prelude::*;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub message: String
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &Props) -> Html {
    let props = props.clone();

    html! {
        <div class="px-4 py-2 rounded-md bg-red-300 text-red-600 border border-red-600 text-center shadow-md">
            {props.message}
        </div>
    }
}