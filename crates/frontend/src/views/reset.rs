use yew::prelude::*;
use crate::components::auth::reset_form::ResetForm;

#[function_component(Reset)]
pub fn login() -> Html {
    html! {
        <div class="col-span-12 row-span-24 flex flex-col justify-center items-center h-full space-y-4">
            <ResetForm />
        </div>
    }
}