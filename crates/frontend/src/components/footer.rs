use yew::prelude::*;
use crate::components::buttons::externalbutton::ExternalButton;

#[function_component(Footer)]
pub fn footer() -> Html {

    html! {
        <footer class="flex flex-row w-screen bg-slate-900 justify-evenly justify-items-center">
            <ExternalButton label="Yew Docs" destination="https://yew.rs/docs/getting-started/introduction"/>
            <ExternalButton label="Are we web yet?" destination="https://www.arewewebyet.org/"/>
        </footer>
    }
}