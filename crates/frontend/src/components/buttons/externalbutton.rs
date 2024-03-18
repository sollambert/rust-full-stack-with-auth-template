use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub destination: String,
    #[prop_or(false)]
    pub disabled: bool
}

#[function_component(ExternalButton)]
pub fn externalbutton(props: &Props) -> Html {
    let props = props.clone();
    let destination = props.destination;
    html! {
        <a href={destination}>
            <div class="flex items-center justify-center
                        text-sm font-medium ring-offset-background
                        transition-colors focus-visible:outline-none
                        focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2
                        disabled:pointer-events-none disabled:opacity-50
                        h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-800"
                    disabled={props.disabled}>
                {props.label}
            </div>
        </a>
    }
}