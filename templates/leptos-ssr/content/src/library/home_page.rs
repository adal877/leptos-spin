use leptos::prelude::ElementChild;
use leptos::*;
use leptos::{prelude::*, task::spawn_local};

use crate::library::save_count;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| {
        set_count.update(|count| *count += 1);
        spawn_local(async move {
            save_count(count.get()).await.unwrap();
        });
    };

    view! {
      <h1>"Welcome to Leptos - served from Spin!"</h1>
      <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[server(prefix = "/api")]
pub async fn save_count(count: u32) -> Result<(), ServerFnError<String>> {
    println!("Saving value {count}");
    let store = spin_sdk::key_value::Store::open_default().map_err(|e| e.to_string())?;
    store
        .set_json("leptos_test_count", &count)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}
