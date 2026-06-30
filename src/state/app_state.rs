use crate::i18n::text::Lang;

#[derive(Clone, Debug)]
pub struct AppState {
    pub lang: Lang,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            lang: Lang::Ja,
        }
    }
}