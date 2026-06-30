use leptos::ev::Event;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Editor(
    text: RwSignal<String>,
    placeholder: impl Fn() -> &'static str + 'static + Copy + Send,
) -> impl IntoView {
    let on_input = move |event: Event| {
        let textarea = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlTextAreaElement>().ok());

        if let Some(textarea) = textarea {
            text.set(textarea.value());
        }
    };

    view! {
        <section class="editor-area">
            <textarea
                class="memo-editor"
                prop:value=move || text.get()
                placeholder=placeholder
                on:input=on_input
            />
        </section>
    }
}