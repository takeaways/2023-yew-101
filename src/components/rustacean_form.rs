use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::use_navigator;

use crate::{
    api::rustaceans::api_rustacean_create,
    components::{alert::Alert, input::Input},
    contexts::CurrentUserContext,
    Route,
};

#[function_component(RustaceanForm)]
pub fn rustacean_form() -> Html {
    let navigator = use_navigator().expect("Navigator not available!");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let name_handle = use_state(String::default);
    let email_handle = use_state(String::default);
    let error_message_handle = use_state(String::default);

    let name = (*name_handle).clone();
    let email = (*email_handle).clone();
    let error_message = (*error_message_handle).clone();

    let name_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handle.set(input.value());
        }
    });

    let email_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            email_handle.set(input.value());
        }
    });

    let cloned_name = name.clone();
    let cloned_email = email.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_name = cloned_name.clone();
        let cloned_email = cloned_email.clone();
        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_current_user_ctx = current_user_ctx.clone();

        match &cloned_current_user_ctx.token {
            Some(token) => {
                let cloned_token = token.clone();
                spawn_local(async move {
                    match api_rustacean_create(
                        &cloned_token,
                        cloned_name.clone(),
                        cloned_email.clone(),
                    )
                    .await
                    {
                        Ok(_) => cloned_navigator.push(&Route::Rustaceans),
                        Err(e) => cloned_error_handle.set(e.to_string()),
                    }
                })
            }
            None => cloned_error_handle.set("Session expired.".to_string()),
        }
    });

    html! {
         <form onsubmit={onsubmit}>
            if error_message.len() > 0{
             <Alert alert_type="danger" message={error_message}/>
            }
            <div class="mb-3">
              <Input
                input_type="text"
                name="name"
                label="이름"
                value={name}
                onchange={name_change}
              />
            </div>
            <div class="mb-3">
              <Input
                input_type="email"
                name="email"
                label="이메일"
                value={email}
                onchange={email_change}
              />
            </div>
            <button type="submit" class="btn btn-primary">{"저장"}</button>
          </form>
    }
}
