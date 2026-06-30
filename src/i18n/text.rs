#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Ja,
    En,
}

pub fn t(lang: Lang, key: &str) -> &'static str {
    match lang {
        Lang::Ja => ja(key),
        Lang::En => en(key),
    }
}

fn ja(key: &str) -> &'static str {
    match key {
        "app_title" => "scratch pad",
        "editor_placeholder" => "ここにメモを入力...",
        "toolbar_tab_list" => "タブ一覧",
        "toolbar_save" => "保存",
        "toolbar_rename" => "タブ名変更",
        "toolbar_settings" => "設定",
        _ => "",
    }
}

fn en(key: &str) -> &'static str {
    match key {
        "app_title" => "scratch pad",
        "editor_placeholder" => "Write your notes here...",
        "toolbar_tab_list" => "Tabs",
        "toolbar_save" => "Save",
        "toolbar_rename" => "Rename",
        "toolbar_settings" => "Settings",
        _ => "",
    }
}