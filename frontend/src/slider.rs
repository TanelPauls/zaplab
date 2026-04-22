use leptos::prelude::*;

#[component]
pub fn Slider(
    label: &'static str,
    min: u32,
    max: u32,
    value: RwSignal<u32>,
) -> impl IntoView {
    view! {
        <div>
            <label>{move || format!("{}: {}", label, value.get())}</label>
            <input
                type="range"
                min=min
                max=max
                prop:value=value
                on:input=move |e| {
                    if let Ok(v) = event_target_value(&e).parse::<u32>() {
                        value.set(v);
                    }
                }
            />
        </div>
    }
}
