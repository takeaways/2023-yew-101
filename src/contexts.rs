use std::rc::Rc;
use yew::prelude::*;
use yew::{Reducible, UseReducerHandle};

use crate::api::user::{LoginResponse, MeResponse, User};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action_type {
            CurrentUserActions::LoginSuccess => {
                let login_resp = action.login_response.expect("Missing login response");
                let me_resp = action.me_response.expect("Missing login response");

                let MeResponse {
                    id,
                    username,
                    created_at,
                } = me_resp;

                Self {
                    user: Some(User {
                        id,
                        username,
                        created_at,
                    }),
                    token: Some(login_resp.token),
                }
                .into()
            }
            CurrentUserActions::LoginFail => Self {
                user: None,
                token: None,
            }
            .into(),
        }
    }
}

pub struct CurrentUserDispatchActions {
    pub action_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>,
}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFail,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);

    html! {
      <ContextProvider<CurrentUserContext> context={user}>
        {props.children.clone()}
      </ContextProvider<CurrentUserContext>>
    }
}
