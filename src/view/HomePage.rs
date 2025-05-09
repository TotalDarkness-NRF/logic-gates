use leptos::{IntoView, SignalUpdate, component, create_signal, view};

/// Renders the home page of your application.
#[component]
pub fn render() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <h2>"Check out logic gates!"</h2>
        <a href="logic-gates">Logic gates</a> 
    }
}