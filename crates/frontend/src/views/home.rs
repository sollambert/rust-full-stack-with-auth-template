use yew::prelude::*;

use crate::components::graphics::ferris::Ferris;

#[function_component(Home)]
pub fn home() -> Html {


    html! {
        <div class="col-span-12 row-span-24 flex justify-center items-center">
            <div class="flex flex-col md:flex-row justify-center items-center
            h-2/3 rounded-md space-y-10
            bg-slate-100 text-slate-800 shadow-md mx-10 p-10
            dark:bg-slate-900 dark:text-slate-100">
                <div class="flex flex-col justify-center text-center space-y-2">
                    <h1 class="text-5xl">{"Welcome!"}</h1>
                    <p>{"This template is built using Yew, Tailwinds, Axum, Sqlx, and Tauri."}</p>
                    <p>{"This project is built to make full stack development with user authentication easily accessible for developers within an entirely Rust ecosystem."}</p>
                    <p>{"In here you'll find some quick demos to get you started!"}</p>
                </div>
                <Ferris />
            </div>
        </div>
    }
}