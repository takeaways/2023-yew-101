use crate::Route;
use yew::prelude::*;
use yew_router::prelude::{use_route, Link};

fn get_nav_classes(current_route: &Route, target_route: Route) -> Classes {
    if *current_route == target_route {
        classes!("nav-link", "active")
    } else {
        classes!("nav-link")
    }
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current_route = use_route::<Route>().expect("No Current route defined");

    html! {
       <nav class="navbar navbar-light">
        <ul class="nav navbar-nav">
          <li class="nav-item">
            <Link<Route> to={Route::Home} classes={get_nav_classes(&current_route, Route::Home)}>
              {"HOME"}
            </Link<Route>>
          </li>
          <li class="nav-item">
            <Link<Route> to={Route::Rustaceans} classes={get_nav_classes(&current_route, Route::Rustaceans)}>
              {"Rustaceans"}
            </Link<Route>>
          </li>
          <li class="nav-item">
            <Link<Route> to={Route::Crates} classes={get_nav_classes(&current_route, Route::Crates)}>
              {"Crates"}
            </Link<Route>>
          </li>
        </ul>
      </nav>
    }
}
