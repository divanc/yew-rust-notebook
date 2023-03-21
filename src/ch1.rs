use yew::prelude::*;

#[function_component(Chapter1)]
pub fn container() -> Html {
    println!("CHAPTER 1");

    html! {
        <section>
        <h1>{"Chapter 1"}</h1>
        <p>{"
            You can do this
        "}</p>

        <code>
            {"println!(\"Hello, world!\");"}
        </code>
        </section>
    }
}
