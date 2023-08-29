use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{contexts::CurrentUserContext, hooks::use_rustaceans, Route};

#[function_component(Crustaceansist)]
pub fn rustacean_list() -> HtmlResult {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let token = current_user_ctx.token.as_ref().expect("Token not found!");
    let rustaceans = use_rustaceans(&token)?;

    Ok(html! {
      <>
        <p>
          <Link<Route> to={Route::RustaceansAdd}>
            {"+ Add new"}
          </Link<Route>>
        </p>
        <table class="table table-striped">
          <thead class="thead-dark">
            <th scope="col">{"아이디"}</th>
            <th scope="col">{"이름"}</th>
            <th scope="col">{"이메일"}</th>
            <th scope="col">{"생성일"}</th>
            <th scope="col">{"Operations"}</th>
          </thead>
          <tbody>
            {
              rustaceans.into_iter().map(|rustacean|{
                html!(
                  <tr>
                    <th scope="row">{rustacean.id}</th>
                    <td>{rustacean.name}</td>
                    <td>{rustacean.email}</td>
                    <td>{rustacean.created_at}</td>
                    <td>
                      <Link<Route> to={Route::RustaceansAdd}>
                        {"✏️ Edit"}
                      </Link<Route>>
                      <span>{"  /  "}</span>
                      <Link<Route> to={Route::RustaceansAdd}>
                        {"🗑️ delete"}
                      </Link<Route>>
                    </td>
                  </tr>
                )
              }).collect::<Vec<Html>>()
            }
          </tbody>
        </table>
      </>
    })
}
