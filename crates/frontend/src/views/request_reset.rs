use yew::prelude::*;
use crate::components::auth::request_reset_form::RequestResetForm;

#[function_component(RequestReset)]
pub fn login() -> Html {
    html! {
        <div class="col-span-12 row-span-24 flex flex-col justify-center items-center h-full space-y-4">
            <RequestResetForm />
        </div>
    }
}