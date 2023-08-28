use pages::{home::Home, login::Login, not_found::NotFound};
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

mod api;
mod components;
mod contexts;
mod pages;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/rustaceans")]
    Rustaceans,
    #[at("/Crates")]
    Crates,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html!(<Login/>),
        Route::Home => html!(<Home />),
        Route::Rustaceans => html!(<Home />),
        Route::Crates => html!(<Home />),
        Route::NotFound => html!(<NotFound />),
    }
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <BrowserRouter>
            <contexts::CurrentUserProvider>
                <Switch<Route> render={switch}/>
            </contexts::CurrentUserProvider>
        </BrowserRouter>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
