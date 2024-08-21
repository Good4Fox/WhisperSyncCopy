use console::Term;
use dialoguer::Select;
use std::process::Command;
use crate::locales::Locale;
use crate::config::{load_config, save_config};
use crate::settings::{settings_delete_file_with_confirmation, change_sync_paths};
use crate::sync::{start_auto_sync, synchronize_paths};

pub async fn show_menu(term: &Term, locale: &Locale) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let options = vec![
            locale.get("menu", "menu_options_0"),
            locale.get("menu", "menu_options_1"),
            locale.get("system", "system_exit")
        ];

        let choice = Select::new()
            .items(&options)
            .default(0)
            .interact()?;

        match choice {
            0 => show_sync_menu(term, locale).await?,
            1 => show_settings_menu(term, locale)?,
            2 => {
                term.write_line(locale.get("msg", "msg_exit"))?;
                break;
            },
            _ => unreachable!(),
        }
    }

    Ok(())
}

pub async fn show_sync_menu(term: &Term, locale: &Locale) -> Result<(), Box<dyn std::error::Error>> {
    let options = vec![
        locale.get("sync", "sync_async"), 
        locale.get("sync", "sync_sync"), 
        locale.get("system", "system_back")
    ];
    
    loop {
        let choice = Select::new()
            .items(&options)
            .default(1)
            .interact()?;

        match choice {
            0 => {
                term.write_line(locale.get("menu", "menu_sync_start"))?;
                start_auto_sync(locale).await?;
            },
            1 => {
                term.write_line(locale.get("menu", "menu_sync_start"))?;
                synchronize_paths(locale).await?;
            },
            2 => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}

pub fn show_settings_menu(term: &Term, locale: &Locale) -> Result<(), Box<dyn std::error::Error>> {
    let options = vec![
        locale.get("settings", "settings_language"), 
        locale.get("settings", "settings_sync_path"), 
        locale.get("settings", "settings_config_del"), 
        locale.get("system", "system_back")
    ];

    loop {
        let choice = Select::new()
            .items(&options)
            .default(1)
            .interact()?;

        match choice {
            0 => change_language(locale, term)?,
            1 => change_sync_paths(locale, term)?,
            2 => settings_delete_file_with_confirmation(locale,"WhisperSyncCopy.toml"),
            3 => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}

pub fn change_language(locale: &Locale, term: &Term) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "WhisperSyncCopy.toml";
    let mut config = load_config(config_path)?;

    let languages = vec!["Русский", "English"];
    let lang_choice = Select::new()
        .items(&languages)
        .default(0)
        .interact()?;

    config.language.current = match lang_choice {
        0 => "ru".to_string(),
        1 => "en".to_string(),
        _ => unreachable!(),
    };

    save_config(&config, config_path)?;
    term.write_line(locale.get("menu", "menu_settings_language_update"))?;
    restart_program()?;

    Ok(())
}

// конфиг

// настройки
pub fn restart_program() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    Command::new(current_exe)
        .spawn()?; 

    std::process::exit(0);
}
