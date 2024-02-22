use leptos::*;
use std::rc::Rc;

/// A simple counter component.
///
/// You can use doc comments like this to document your component.
#[component]
pub fn Input() -> impl IntoView {
    let (name, set_name) = create_signal(Some("Controlled".to_string()));

    view! {
        {move || {
            name.get()
                .map(|name| {
                    view! {
                        <input
                            type="text"
                            on:input=move |ev| {
                                set_name(Some(event_target_value(&ev)));
                            }

                            // the `prop:` syntax lets you update a DOM property,
                            // rather than an attribute.
                            prop:value=name
                        />
                    }
                })
        }}
    }
}
