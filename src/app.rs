use leptos::prelude::*;

use crate::components::editor::Editor;
use crate::components::toolbar::Toolbar;
use crate::i18n::text::{t, Lang};
use crate::state::app_state::AppState;

pub fn App() -> impl IntoView {
    let state = RwSignal::new(AppState::new());

    let memo_text = RwSignal::new(String::new());

    let on_open_tab_list = move |_| {
        // フェーズ3で実装予定
        // 現時点ではスケルトン
        web_sys::console::log_1(&"open tab list".into());
    };

    let on_save = move |_| {
        // フェーズ4で実装予定
        web_sys::console::log_1(&"save".into());
    };

    let on_rename_tab = move |_| {
        // フェーズ3で実装予定
        web_sys::console::log_1(&"rename tab".into());
    };

    let on_open_settings = move |_| {
        // フェーズ5で実装予定
        web_sys::console::log_1(&"open settings".into());
    };

    view! {
        <main class="app-shell">
            <header class="app-header">
                <div class="app-title">
                    {move || t(state.get().lang, "app_title")}
                </div>
            </header>

            <Toolbar
                lang=move || state.get().lang
                on_open_tab_list=on_open_tab_list
                on_save=on_save
                on_rename_tab=on_rename_tab
                on_open_settings=on_open_settings
            />

            <Editor
                text=memo_text
                placeholder=move || t(state.get().lang, "editor_placeholder")
            />
        </main>
    }
}