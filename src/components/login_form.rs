use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::use_navigator;

use crate::{
    api::user::{api_login, api_me, LoginResponse, MeResponse},
    components::{alert::Alert, input::Input},
    contexts::{CurrentUserActions, CurrentUserContext, CurrentUserDispatchActions},
    Route,
};

async fn login(
    username: String,
    password: String,
) -> Result<(LoginResponse, MeResponse), gloo_net::Error> {
    let login_response = api_login(username, password).await?;
    let me_response = api_me(&login_response.token).await?;
    Ok((login_response, me_response))
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator();
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let username_handle = use_state(String::default);
    let password_handle = use_state(String::default);
    let error_message_handle = use_state(String::default);

    let username = (*username_handle).clone();
    let password = (*password_handle).clone();
    let error_message = (*error_message_handle).clone();

    let username_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            username_handle.set(input.value());
        }
    });

    let password_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            password_handle.set(input.value());
        }
    });

    let cloned_username = username.clone();
    let cloned_password = password.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_username = cloned_username.clone();
        let cloned_password = cloned_password.clone();
        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_current_user_ctx = current_user_ctx.clone();

        spawn_local(async move {
            match login(cloned_username.clone(), cloned_password.clone()).await {
                Ok(responses) => {
                    cloned_current_user_ctx.dispatch(CurrentUserDispatchActions {
                        action_type: CurrentUserActions::LoginSuccess,
                        login_response: Some(responses.0),
                        me_response: Some(responses.1),
                    });

                    if let Some(nav) = cloned_navigator {
                        nav.push(&Route::Home);
                    }
                }
                Err(e) => cloned_error_handle.set(e.to_string()),
            }
        })
    });

    html! {
         <form onsubmit={onsubmit}>
            if error_message.len() > 0{
             <Alert alert_type="danger" message={error_message}/>
            }
            <div class="mb-3">
              <Input
                input_type="text"
                name="username"
                label="아이디"
                value={username}
                onchange={username_change}
              />
            </div>
            <div class="mb-3">
              <Input
                input_type="password"
                name="password"
                label="비밀번호"
                value={password}
                onchange={password_change}
              />
            </div>
            <button type="submit" class="btn btn-primary">{"로그인"}</button>
          </form>
    }
}
