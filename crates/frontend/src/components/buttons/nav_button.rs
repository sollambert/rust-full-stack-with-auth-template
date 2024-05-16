use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub destination: AppRoute,
    #[prop_or(false)]
    pub disabled: bool
}

#[function_component(NavButton)]
pub fn navbutton(props: &Props) -> Html {
    let props = props.clone();
    let destination = props.destination;
    html! {
        <Link<AppRoute> to={destination}>
            <div class="flex items-center justify-center h-full
                    text-md font-medium ring-offset-background
                    transition-colors focus-visible:outline-none
                    focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2
                    disabled:pointer-events-none disabled:opacity-50 px-4 py-2
                    bg-slate-100 text-slate-800 hover:bg-slate-200
                    dark:bg-slate-900 dark:text-slate-100 dark:hover:bg-slate-800"
                    disabled={props.disabled}>
                {props.label}
            </div>
        </Link<AppRoute>>
    }
}