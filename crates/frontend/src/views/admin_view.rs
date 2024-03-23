use yew::prelude::*;

use crate::components::users_table::UsersTable;

#[function_component(AdminView)]
pub fn admin_view() -> Html {

    html! {
        <main class="flex flex-col items-center h-full">
            <UsersTable />
        </main>
    }
}