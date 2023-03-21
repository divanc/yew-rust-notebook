use yew::prelude::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};

mod ch1;
mod home;
mod routes;

use routes::Route;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home::Home /> },
        Route::Chapter1 => html! { <ch1::Chapter1 /> },
    }
}

#[function_component(App)]
fn container() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
