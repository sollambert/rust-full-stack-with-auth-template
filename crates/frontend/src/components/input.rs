use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: String,
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or(false)]
    pub disabled: bool
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let props = props.clone();
    let callback = props.oninput;
    let oninput = Callback::from(move |ev: InputEvent| {
        ev.prevent_default();
        callback.emit(ev);
    });
    html! {
        <>
            <input class="rounded-md text-sm font-medium ring-offset-background
                        transition-colors focus-visible:outline-none
                        focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2
                        disabled:pointer-events-none disabled:opacity-50
                        h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90"
                    oninput={oninput}
                    type={props.input_type}
                    placeholder={props.placeholder}
                    disabled={props.disabled}
                    value={props.value}/>
        </>
    }
}