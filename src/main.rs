use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!(
        <div>
            <h2>{"Hello world"}</h2>
            <p>{"this is working property"}</p>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
