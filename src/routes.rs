use yew_router::prelude::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/chap-1")]
    Chapter1,
}
