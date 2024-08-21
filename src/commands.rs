use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "WhisperSyncCopy", about = "Программа для синхронизации и настройки")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Test,
    Run,
    Sync {
        #[arg(short = 's', long = "source")]
        source: String,
        
        #[arg(short = 'd', long = "destination")]
        destination: String,
    },
    Osync,
}

impl Cli {
    pub fn handle_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Some(Commands::Test) => {
                println!("Запуск тестовой команды...");
            },
            Some(Commands::Sync { source, destination }) => {
                println!("Получены пути для синхронизации:");
                println!("  Исходный путь: {}", source);
                println!("  Целевой путь: {}", destination);
            },
            Some(Commands::Run) => {
                println!("Запуск программы...");
            },
            Some(Commands::Osync) => {
                println!("Запуск программы и открытие меню синхронизации...");
            },
            None => {
                return Ok(());
            }
        }
        Ok(())
    }
}
