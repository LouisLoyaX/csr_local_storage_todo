use leptos::*;
mod app;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/>})
}
