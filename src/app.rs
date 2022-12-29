use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> Element {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    // TODO: this doesn't exist in 0.1.0-alpha -- wait for the next/beta release and uncomment!
    // provide_meta_context(cx);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        cx,
        <main>
            <Title text="cargo-leptos starter"/>
            <Stylesheet href="/style.css"/>
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
        </main>
    }
}
