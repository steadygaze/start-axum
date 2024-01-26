pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_binden::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use leptos::*;
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
