use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: AttrValue,
}

#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    html! {
        <div class="alert alert-danger" role="alert">{props.message.clone()}</div>
    }
}
