use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Language {
    Rust = "rust",
    Swift = "swift",
}

impl TryFrom<Language> for tser::Language {
    type Error = String;
    fn try_from(value: Language) -> Result<Self, Self::Error> {
        Ok(match value {
            Language::Swift => tser::Language::Swift,
            Language::Rust => tser::Language::Rust,
            other => return Err(format!("Invalid language: {}", other.to_str())),
        })
    }
}

#[wasm_bindgen]
pub fn generate_from_ts(ts_src: &str, lang: Language) -> Result<String, String> {
    console_error_panic_hook::set_once();
    tser::generate_from_ts(ts_src, lang.try_into()?).map_err(|err| err.to_string())
}
