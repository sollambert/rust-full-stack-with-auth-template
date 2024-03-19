use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;
use types::user::UserInfo;

use crate::{components::{auth::{admin_route::AdminRoute, protected_route::ProtectedRoute}, footer::Footer, header::Header}, services, views::{admin_view::AdminView, home::Home, login::Login, not_found::NotFound, register::Register, user_view::UserView}};


#[derive(Default, PartialEq, Store)]
pub struct UserState {
    pub user_info: UserInfo
}

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/admin")]
    AdminPanel,
    #[at("/settings")]
    UserPanel,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::AdminPanel => html! {<AdminRoute><AdminView /></AdminRoute>},
        AppRoute::UserPanel => html! {<ProtectedRoute><UserView /></ProtectedRoute>},
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<Register />},
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let (user_state, user_dispatch) = use_store::<UserState>();

    use_effect(move || {
        if user_state.user_info.uuid == String::new() {
            yew::platform::spawn_local(async move {
                let user_info = services::user::get_user_info().await;
                user_dispatch.set(UserState {user_info});
            });
        }
    });

    html! {
        <HashRouter>
            <body class="flex flex-col place-content-between h-screen w-screen">
                <Header />
                <Switch<AppRoute> render={switch} />
                <Footer />
            </body>
        </HashRouter>
    }
}
