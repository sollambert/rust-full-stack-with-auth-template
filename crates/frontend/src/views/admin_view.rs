use yew::prelude::*;

use crate::components::users_table::UsersTable;

#[function_component(AdminView)]
pub fn admin_view() -> Html {

    html! {
        <main class="col-span-12 row-span-24 flex flex-col items-center">
            <UsersTable />
        </main>
    }
}