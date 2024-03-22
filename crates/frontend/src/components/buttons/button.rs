use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub label: String,
    #[prop_or(html!())]
    pub icon: Html,
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub disabled: bool
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
        <button class="inline-flex items-center justify-center
                    rounded-md text-sm font-medium ring-offset-background
                    transition-colors focus-visible:outline-none
                    focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2
                    disabled:pointer-events-none disabled:opacity-50
                    px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90"
                onclick={onclick}
                disabled={props.disabled}>
            {props.label}
            {props.icon}
        </button>
    }
}