use yew::prelude::*;

use crate::components::{header::Header, rustacean_list::Crustaceansist, sidebar::Sidebar};

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
    html! {
      <div class="container">
        <div class="row">
          <div class="col-sm-auto">
            <Sidebar/>
          </div>
          <div class="col-sm-auto mt-3">
            <Header/>
            <Crustaceansist/>
          </div>
        </div>
      </div>
    }
}
