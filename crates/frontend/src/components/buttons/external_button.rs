use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub destination: String
}

#[function_component(ExternalButton)]
pub fn externalbutton(props: &Props) -> Html {
    let props = props.clone();
    let destination = props.destination;
    html! {
        <a class="flex items-center justify-center
                text-md font-medium ring-offset-background
                transition-colors focus-visible:outline-none
                focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2
                h-full px-4 py-2
                bg-slate-100 text-slate-800 hover:bg-slate-200
                dark:bg-slate-900 dark:text-slate-100 dark:hover:bg-slate-800"
            href={destination}>
                {props.label}
        </a>
    }
}