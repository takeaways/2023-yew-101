use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
       <nav class="navbar navbar-light">
        <ul class="nav navbar-nav">
          <li class="nav-item">
            <Link<Route> to={Route::Home}>{"HOME"}</Link<Route>>
          </li>
          <li class="nav-item">
            <Link<Route> to={Route::Rustaceans}>{"Rustaceans"}</Link<Route>>
          </li>
          <li class="nav-item">
            <Link<Route> to={Route::Crates}>{"Crates"}</Link<Route>>
          </li>
        </ul>
      </nav>
    }
}
