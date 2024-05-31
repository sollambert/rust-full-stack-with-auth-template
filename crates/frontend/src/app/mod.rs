use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::{auth::{admin_route::AdminRoute, protected_route::ProtectedRoute}, footer::Footer, header::Header}, views::{admin_view::AdminView, chat::Chat, home::Home, login::Login, not_found::NotFound, register::Register, user_view::UserView}};
use crate::hooks::use_user_info;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/admin")]
    AdminPanel,
    #[at("/settings")]
    UserPanel,
    #[at("/chat")]
    Chat,
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
        AppRoute::Chat => html! {<ProtectedRoute><Chat /></ProtectedRoute>},
        AppRoute::UserPanel => html! {<ProtectedRoute><UserView /></ProtectedRoute>},
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<Register />},
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let _user_info = use_user_info();

    html! {
        <HashRouter>
            <body class="flex flex-col space-between w-screen h-screen bg-slate-50 dark:bg-slate-700 overflow-hidden">
                <Header />
                <main class="grid grid-rows-24 grid-cols-12 auto-rows-auto h-full w-full py-12">
                    <Switch<AppRoute> render={switch} />
                </main>
                <Footer />
            </body>
        </HashRouter>
    }
}
