use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::components::sidebar::Sidebar;
use crate::{api::user::User, contexts::CurrentUserContext, Route};

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    match &current_user_ctx.user {
        Some(user) => {
            let User {
                id,
                created_at,
                username,
            } = user;

            html! {
              <div class="container">
                <div class="row">
                  <div class="col-am-auto">
                    <Sidebar/>
                  </div>
                  <div class="col-am-auto">
                    {id}{username}{created_at}
                  </div>
                </div>
              </div>
            }
        }
        None => html!(
          <Redirect<Route> to={Route::Login}/>
        ),
    }
}
