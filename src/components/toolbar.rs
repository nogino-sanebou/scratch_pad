use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::i18n::text::{t, Lang};

#[component]
pub fn Toolbar(
    lang: impl Fn() -> Lang + 'static + Copy + Send,
    on_open_tab_list: impl Fn(MouseEvent) + 'static + Copy,
    on_save: impl Fn(MouseEvent) + 'static + Copy,
    on_rename_tab: impl Fn(MouseEvent) + 'static + Copy,
    on_open_settings: impl Fn(MouseEvent) + 'static + Copy,
) -> impl IntoView {
    view! {
        <nav class="toolbar">
            <button
                type="button"
                class="toolbar-button"
                on:click=on_open_tab_list
            >
                {move || t(lang(), "toolbar_tab_list")}
            </button>

            <button
                type="button"
                class="toolbar-button"
                on:click=on_save
            >
                {move || t(lang(), "toolbar_save")}
            </button>

            <button
                type="button"
                class="toolbar-button"
                on:click=on_rename_tab
            >
                {move || t(lang(), "toolbar_rename")}
            </button>

            <button
                type="button"
                class="toolbar-button"
                on:click=on_open_settings
            >
                {move || t(lang(), "toolbar_settings")}
            </button>
        </nav>
    }
}