use leptos::*;
use spin_sdk::key_value;
use thiserror::Error;

/// Server function for saving the count.
#[server(prefix = "{{http-path}}/api")]
pub async fn save_count(count: u32) -> Result<(), ServerFnError<String>> {
    println!("Saving value {count}");
    let store = key_value::Store::open_default().map_err(|e| e.to_string())?;
    store
        .set_json("{{project-name | snake_case}}_count", &count)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}
