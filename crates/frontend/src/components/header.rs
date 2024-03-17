use yew::prelude::*;
use crate::components::buttons::navbutton::NavButton;

#[function_component(Header)]
pub fn header() -> Html {

    html! {
        <header class="flex flex-row bg-slate-900 justify-between justify-items-center">
            <NavButton label="Home" destination="/"/>
            <div class="flex flex-row">
                <NavButton label="Login" destination="/login"/>
                <NavButton label="Register" destination="/register"/>
            </div>
        </header>
    }
}