pub mod app {
    use crate::pages::index::Index;
    use yew::prelude::*;

    #[function_component(App)]
    pub fn app() -> Html {
        html! {
            <div>
                <Index />
            </div>
        }
    }
}
