use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (message, set_message) = signal(String::from("Loading..."));

    leptos::task::spawn_local(async move {
        if let Ok(resp) = gloo_net::http::Request::get("/api/").send().await {
            if let Ok(data) = resp.json::<serde_json::Value>().await {
                if let Some(msg) = data["message"].as_str() {
                    set_message.set(msg.to_string());
                }
            }
        }
    });

    view! {
        <h1>"WELCOME"</h1>
        <p>{message}</p>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}
