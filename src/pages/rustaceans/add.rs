use yew::prelude::*;

use crate::components::{
    header::Header, rustacean_form::RustaceanForm, rustacean_list::Crustaceansist, sidebar::Sidebar,
};

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    html! {
      <div class="container">
        <div class="row">
          <div class="col-sm-auto">
            <Sidebar/>
          </div>
          <div class="col-sm-auto mt-3">
            <Header/>
            <RustaceanForm/>
          </div>
        </div>
      </div>
    }
}
