use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::{Link, Redirect};

use crate::{api::rustaceans::api_rustaceans, contexts::CurrentUserContext, Route};

#[function_component(Crustaceansist)]
pub fn rustacean_list() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    match &current_user_ctx.token {
        Some(token) => {
            let cloned_token = token.clone();

            spawn_local(async move {
                api_rustaceans(&cloned_token).await;
            });

            html! {
              <>
                <p>
                  <Link<Route> to={Route::RustaceansAdd}>
                    {"+ Add new"}
                  </Link<Route>>
                </p>
                <table class="table">
                  <thead>
                    <th>{"아이디"}</th>
                    <th>{"이름"}</th>
                    <th>{"이메일"}</th>
                    <th>{"생성일"}</th>
                    <th>{"Operations"}</th>
                  </thead>
                  <tbody>

                  </tbody>
                </table>
              </>
            }
        }
        None => html!(
          <Redirect<Route> to={Route::Login}/>
        ),
    }
}
