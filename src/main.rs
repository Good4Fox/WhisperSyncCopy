mod config;
mod menu;
mod locales;
mod commands;
mod settings;
mod sync;

use console::Term;
use std::fs;
use tokio;
use clap::Parser;
use locales::Locale;
use commands::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Err(e) = cli.handle_command() {
        eprintln!("Ошибка: {}", e);
        return Err(e);
    }

    match cli.command {
        None | Some(commands::Commands::Run) => {
            let term = Term::stdout();
            term.clear_screen()?;

            let config_path = "WhisperSyncCopy.toml";
            let config: config::Config;

            if fs::metadata(config_path).is_ok() {
                config = config::load_config(config_path)?;
            } else {
                config = config::create_new_config(config_path)?;
            }

            let locale = Locale::new(&config.language.current)?;

            let welcome_message = locale.get("menu", "menu_welcome");
            term.write_line(welcome_message)?;

            menu::show_menu(&term, &locale).await?;
        }
        Some(commands::Commands::Osync) => {
            let term = Term::stdout();
            term.clear_screen()?;

            let config_path = "WhisperSyncCopy.toml";
            let config: config::Config;

            if fs::metadata(config_path).is_ok() {
                config = config::load_config(config_path)?;
            } else {
                config = config::create_new_config(config_path)?;
            }

            let locale = Locale::new(&config.language.current)?;

            let welcome_message = locale.get("menu", "menu_welcome");
            term.write_line(welcome_message)?;

            menu::show_sync_menu(&term, &locale).await?;
            menu::show_menu(&term, &locale).await?;
        }
        _ => {}
    }

    Ok(())
}
