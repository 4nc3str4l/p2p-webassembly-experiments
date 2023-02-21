use yew::prelude::*;

#[function_component]
fn HelloWorld() -> Html {
    html! { <h1> { "Hello world" } </h1> }
}

#[function_component]
fn Explanation() -> Html {
    html! { <p> { "This is my first contact with yew & webassembly in Rust!" }</p> }
}

#[function_component]
fn App() -> Html {
    html! { 
        <>
            <HelloWorld /> 
            <Explanation />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}