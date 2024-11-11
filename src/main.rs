#![allow(non_snake_case)]

use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let icon = "https://media.tenor.com/4yqdl2fFYwsAAAAM/really.gif";

    view! {
        <Icon
            // Only errors if we use the `icon` variable/ident. If we put the string directly here, it works
            icon=icon
            {..}
            draggable="true" // This isn't needed, it still errors with no attributes after `{..}`
        />
    }
}

#[component]
fn Icon(icon: &'static str) -> impl IntoView {
    view! {
        <img src=icon />
    }
}
