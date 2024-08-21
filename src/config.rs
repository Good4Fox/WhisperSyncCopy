use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub language: Language,
    pub paths: Paths,
    pub getignore: GetIgnore,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    pub current: String,
}

#[derive(Serialize, Deserialize)]
pub struct Paths {
    pub source: String,
    pub destination: String,
    pub save_paths: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetIgnore {
    pub patterns: Vec<String>,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}
    
pub fn save_config(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::ser::to_string(&config)?;
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

pub fn create_new_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    use dialoguer::Select;

    let languages = vec!["Русский", "English"];
    let lang_choice = Select::new()
        .items(&languages)
        .default(0)
        .interact()?;

    let config = Config {
        language: Language {
            current: match lang_choice {
                0 => "ru".to_string(),
                1 => "en".to_string(),
                _ => unreachable!(),
            },
        },
        paths: Paths {
            source: "".to_string(),
            destination: "".to_string(),
            save_paths: true,
        },
        getignore: GetIgnore {
            patterns: vec![],
        },
    };

    save_config(&config, path)?;

    Ok(config)
}
