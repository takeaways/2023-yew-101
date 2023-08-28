use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::{
    api::user::User,
    contexts::{CurrentUserActions, CurrentUserContext, CurrentUserDispatchActions},
    Route,
};

#[function_component(Header)]
pub fn header() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    match &current_user_ctx.user {
        Some(user) => {
            let cloned_current_user_ctx = current_user_ctx.clone();

            let onclick = Callback::from(move |_: MouseEvent| {
                cloned_current_user_ctx.dispatch(CurrentUserDispatchActions {
                    action_type: CurrentUserActions::LoginFail,
                    login_response: None,
                    me_response: None,
                });
            });

            let User { username, .. } = user;
            html!(
             <div class="text-end">
                <p>
                  <span class="pe-1">{"Welcome, "}{username.clone()}</span>
                  <button class="btn btn-danger" onclick={onclick}>{"로그아웃"}</button>
                </p>
             </div>
            )
        }
        None => html!(
          <Redirect<Route> to={Route::Login}/>
        ),
    }
}
