use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>{"Hello world"}</div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
