mod app;
mod components;
mod functions;
mod menu;
mod rebalancer;
mod types;
include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

use app::*;
use leptos::prelude::*;
use leptos_use::docs::demo_or_body;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = mount_to(demo_or_body(), || {
        view! { <App /> }
    });

    unmount_handle.forget();
}
