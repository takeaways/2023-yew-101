use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::input::Input;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username_handle = use_state(String::default);
    let password_handle = use_state(String::default);

    let username = (*username_handle).clone();
    let password = (*password_handle).clone();

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
        log!("Submmiting...");
        log!(cloned_username.clone());
        log!(cloned_password.clone());
    });

    html! {
         <form onsubmit={onsubmit}>
            <div class="mb-3">
              <Input
                input_type="text"
                name="username"
                label="Username"
                value={username}
                onchange={username_change}
              />
            </div>
            <div class="mb-3">
              <Input
                input_type="password"
                name="password"
                label="Password"
                value={password}
                onchange={password_change}
              />
            </div>
            <button type="submit" class="btn btn-primary">{"Login"}</button>
          </form>
    }
}
