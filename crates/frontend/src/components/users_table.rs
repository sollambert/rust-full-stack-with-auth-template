use types::user::UserInfo;
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};

use crate::{services, components::buttons::button::Button};

#[function_component(UsersTable)]
pub fn users_table() -> Html {
    let users = use_state(|| Vec::<UserInfo>::new());
    let delete_user_uuid = use_state(|| String::new());

    let handle_get_users = {
        let users = users.clone();
        use_async(async move {
            let response = services::user::get_all_users().await;
            match response {
                Ok(data) => {
                    users.set(data.1);
                    Ok(data.0)
                },
                Err(error) => {
                    Err(error)
                }
            }
        })
    };

    let handle_get_users_clone = handle_get_users.clone();
    let delete_user_uuid_clone = delete_user_uuid.clone();
    let handle_delete = use_async(async move {
        let delete_user_uuid = delete_user_uuid_clone;
        let response = services::user::delete_user((*delete_user_uuid).to_string()).await;
        match response {
            Ok(data) => {
                handle_get_users_clone.run();
                Ok(data)
            },
            Err(error) => {
                Err(error)
            }
        }
    });

    let delete_user_uuid_clone = delete_user_uuid.clone();
    let onclick = Callback::from(move |uuid| {
        delete_user_uuid_clone.set(uuid);
        handle_delete.run();
    });

    let handle_get_users_clone = handle_get_users.clone();
    use_effect_once(move || {
        handle_get_users_clone.run();
        move || {}
    });

    html! {
        <div class="w-11/12 flex flex-col h-min
        rounded-md text-lg font-strong overflow-y-auto
        border-slate-300 dark:border-slate-700 border
        h-10 px-4 py-2 my-10
        bg-slate-100 text-slate-800 shadow-md
        dark:bg-slate-900 dark:text-slate-100">
            <table>
                <thead>
                    <tr class="text-left">
                        <th>{"UUID"}</th>
                        <th>{"Username"}</th>
                        <th>{"Email"}</th>
                        <th>{"Admin"}</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    { (*users).clone().into_iter().map(|user: UserInfo| {
                        let onclick = onclick.clone();
                        let delete_id = user.uuid.clone();
                        html!{
                            <tr>
                                <td>{user.uuid}</td>
                                <td>{user.username}</td>
                                <td>{user.email}</td>
                                <td>{user.is_admin.to_string()}</td>
                                <td><Button color="bg-slate-200 text-slate-800 hover:bg-slate-300 dark:bg-slate-800 dark:text-slate-100 dark:hover:bg-slate-700"
                                        label="Delete" onclick={move |_| {onclick.emit(delete_id.clone());}}/></td>
                            </tr>
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}