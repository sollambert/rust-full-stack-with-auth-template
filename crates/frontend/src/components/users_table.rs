use types::user::UserInfo;
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use gloo_console::error;

use crate::{services, components::buttons::button::Button};

#[function_component(UsersTable)]
pub fn users_table() -> Html {
    let users = use_state(|| Vec::<UserInfo>::new());

    let get_users = {
        let users = users.clone();
        use_async(async move {
            let response = services::user::get_all_users().await;
            match response {
                Ok(data) => {
                    users.set(data.1);
                    Ok(data.0)
                },
                Err(error) => {
                    error!("No response found: {}", error.to_string());
                    Err(error)
                }
            }
        })
    };

    use_effect_once(move || {
        let get_users = get_users.clone();
        get_users.run();
        move || {}
    });

    html! {
        <div class="w-11/12 flex flex-col h-min
        rounded-md text-lg font-strong overflow-y-auto
        h-10 px-4 py-2 my-10 bg-slate-900 text-slate-100">
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
                        html!{
                            <tr>
                                <td>{user.uuid}</td>
                                <td>{user.username}</td>
                                <td>{user.email}</td>
                                <td>{user.is_admin.to_string()}</td>
                                <td><Button label="Delete" onclick={|_|{}}/></td>
                            </tr>
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}