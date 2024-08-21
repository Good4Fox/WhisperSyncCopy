use std::collections::HashMap;
use std::error::Error;

pub struct Locale {
    pub messages: HashMap<String, HashMap<String, String>>,
}

impl Locale {
    pub fn new(language: &str) -> Result<Self, Box<dyn Error>> {
        let locale_str = match language {
            "ru" => include_str!("locales/ru.toml"),
            "en" => include_str!("locales/en.toml"),
            _ => include_str!("locales/en.toml"),
        };

        let messages: HashMap<String, HashMap<String, String>> = toml::from_str(locale_str)?;
        Ok(Self { messages })
    }

    pub fn get(&self, section: &str, key: &str) -> &str {
        self.messages
            .get(section)
            .and_then(|s| s.get(key))
            .map(|v| v.as_str())
            .unwrap_or("")
    }
}
