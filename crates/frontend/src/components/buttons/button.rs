use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub label: String,
    #[prop_or(html!())]
    pub icon: Html,
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(String::from("bg-slate-100 text-slate-800 hover:bg-slate-200
    dark:bg-slate-900 dark:text-slate-100 dark:hover:bg-slate-800"))]
    pub color: String
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let props = props.clone();
    let callback = props.onclick;
    let onclick = Callback::from(move |ev: MouseEvent| {
        ev.prevent_default();
        callback.emit(ev);
    });
    html! {
        <button class={"inline-flex items-center justify-center
                    rounded-md text-sm font-medium ring-offset-background
                    border-slate-300 dark:border-slate-800 border
                    transition-colors focus-visible:outline-none
                    focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2
                    disabled:pointer-events-none disabled:opacity-50 px-4 py-2 shadow-md ".to_owned() + &props.color}
                onclick={onclick}
                disabled={props.disabled}>
            {props.label}
            {props.icon}
        </button>
    }
}