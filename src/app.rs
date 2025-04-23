use leptos::{IntoView, component, view};
use leptos_meta::*;
use leptos_router::*;
use crate::view::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/logic-gates.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage::Render/>
                    <Route path="logic-gates" view=LogicGates::Render/>
                    <Route path="/*any" view=NotFound::Render/>
                </Routes>
            </main>
        </Router>
    }
}