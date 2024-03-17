use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;
use types::user::UserInfo;

use crate::{components::{footer::Footer, header::Header}, views::{home::Home, login::Login, notfound::NotFound, register::Register}};


#[derive(Default, PartialEq, Store)]
pub struct UserState {
    pub user_info: UserInfo
}

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<Register />},
        AppRoute::Home => html! {<Home />},
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {

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
