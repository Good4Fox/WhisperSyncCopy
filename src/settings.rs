use std::fs;
use std::io::{self, Write};
use dialoguer::Input;
use std::path::Path;
use std::process;
use console::Term;

use crate::locales::Locale;
use crate::config::{load_config, save_config};

pub fn settings_delete_file_with_confirmation(locale: &Locale, file_path: &str) {
    if Path::new(file_path).exists() {
        print!("{} '{}'? (Y/N): ", locale.get("settings", "settings_confirmation"), file_path);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "y" || input == "yes" {
            match fs::remove_file(file_path) {
                Ok(_) => println!("{} '{}'.", locale.get("settings", "settings_file_delete"), file_path),
                Err(e) => println!("{} '{}'", locale.get("settings", "settings_file_delete_error"), e),
            }
            process::exit(0);
        } else {
            println!("{}", locale.get("settings", "settings_file_delete_canceled"));
        }
    } else {
        println!("{} '{}'.", locale.get("settings", "settings_file_delete_none"), file_path);
    }
}

pub fn change_sync_paths(locale: &Locale, term: &Term) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "WhisperSyncCopy.toml";
    let mut config = load_config(config_path)?;

    let source_path: String = Input::new()
        .with_prompt(locale.get("settings", "settings_file_new_source"))
        .default(config.paths.source.clone())
        .interact_text()?;

    let destination_path: String = Input::new()
        .with_prompt(locale.get("settings", "settings_file_new_destination"))
        .default(config.paths.destination.clone())
        .interact_text()?;

    config.paths.source = source_path;
    config.paths.destination = destination_path;

    save_config(&config, config_path)?;
    term.write_line(locale.get("settings", "settings_sync_successfully"))?;

    Ok(())
}