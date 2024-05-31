use yew::prelude::*;
use crate::components::buttons::external_button::ExternalButton;

#[function_component(Footer)]
pub fn footer() -> Html {

    html! {
        <footer class="flex flex-row justify-self-end w-screen bg-slate-100 dark:bg-slate-900 z-10
                bottom-0 fixed w-screen border-slate-300 dark:border-slate-700 border-t
                justify-evenly justify-items-center items-center shadow-md h-12">
            <ExternalButton label="Yew Docs" destination="https://yew.rs/docs/getting-started/introduction"/>
            <ExternalButton label="Are we web yet?" destination="https://www.arewewebyet.org/"/>
        </footer>
    }
}