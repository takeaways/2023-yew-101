use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div class="container">
        <div class="row">
          <div class="col-sm-auto">
            <Sidebar/>
          </div>
          <div class="col-sm-auto mt-3">
            <Header/>
          </div>
        </div>
      </div>
    }
}
