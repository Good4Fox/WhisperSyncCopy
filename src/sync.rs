use std::fs;
use std::io::{self};
use std::time::SystemTime;
use std::path::Path;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use dialoguer::{Input, theme::ColorfulTheme, Select};
use notify::{RecommendedWatcher, RecursiveMode, EventKind, Watcher, Config};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::mpsc;
use crate::locales::Locale;
use std::pin::Pin;
use crate::config::{load_config, save_config};

pub async fn start_auto_sync(locale: &Locale) -> Result<(), Box<dyn std::error::Error>> {
    let should_continue = Arc::new(AtomicBool::new(true));

    // Загрузка конфигурации для получения путей
    let config_path = "WhisperSyncCopy.toml";
    let config = load_config(config_path)?;

    // Канал для получения событий изменения файлов
    let (tx, mut rx) = mpsc::channel(1);

    // Создаем наблюдателя за файловой системой с конфигурацией
    let mut watcher = RecommendedWatcher::new(move |res| {
        if let Ok(event) = res {
            let _ = tx.try_send(event);
        }
    }, Config::default())?;

    // Используем загруженный путь источника из конфигурации
    let source = Path::new(&config.paths.source);
    watcher.watch(source, RecursiveMode::Recursive)?;

    while should_continue.load(Ordering::Relaxed) {
        clear_console()?;

        // Ожидаем события изменения файлов
        if let Some(event) = rx.recv().await {
            match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                    // Выполняем синхронизацию
                    if let Err(e) = synchronize_paths(locale).await {
                        println!("{}: {}", locale.get("sync", "sync_async_error_occurred"), e);
                    }
                }
                _ => {}
            }
        }

        // Выводим меню с опциями "Продолжить" и "Остановить"
        let options = vec![
            locale.get("sync", "sync_async_continue").to_string(),
            locale.get("sync", "sync_async_stop").to_string(),
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(locale.get("sync", "sync_async_select_action"))
            .items(&options)
            .default(0)
            .interact()?;

        // Если выбрано "Остановить", выходим из цикла
        if selection == 1 {
            should_continue.store(false, Ordering::Relaxed);
        }
    }

    println!("{}", locale.get("sync", "sync_async_stopped"));
    Ok(())
}

pub async fn synchronize_paths(locale: &Locale) -> Result<(), Box<dyn std::error::Error>> {
    clear_console()?;

    let config_path = "WhisperSyncCopy.toml";
    let mut config = load_config(config_path)?;

    while config.paths.source.is_empty() || config.paths.destination.is_empty() || 
            !Path::new(&config.paths.source).exists() || !Path::new(&config.paths.destination).exists() {
        config = prompt_for_paths(locale, config).await?;
        save_config(&config, config_path)?;
    }

    let source = Path::new(&config.paths.source);
    let destination = Path::new(&config.paths.destination);

    let gitignore = load_gitignore(&config, source)?;
    let all_files_up_to_date = check_files_up_to_date(source, destination)?;

    if all_files_up_to_date {
        println!("{}", locale.get("sync", "sync_actual"));
        return Ok(());
    }

    let total_files = count_files(source)?;
    let progress = ProgressBar::new(total_files);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {percent}% ({eta})")
            .unwrap()
    );
    let message_sync_files_sync = locale.get("sync", "sync_files_sync").to_string();
    progress.set_message(message_sync_files_sync);

    copy_recursively(locale, source, destination, &progress, &gitignore).await?;

    let message_sync_files_sync_completed = locale.get("sync", "sync_files_sync_completed").to_string();
    progress.finish_with_message(message_sync_files_sync_completed);

    Ok(())
}


fn clear_console() -> io::Result<()> {
    let term = console::Term::stdout();
    term.clear_screen()?;
    Ok(())
}

async fn prompt_for_paths(locale: &Locale, mut config: crate::config::Config) -> Result<crate::config::Config, Box<dyn std::error::Error>> {
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

    Ok(config)
}

fn check_files_up_to_date(source: &Path, destination: &Path) -> io::Result<bool> {
    let mut all_files_up_to_date = true;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = destination.join(entry.file_name());

        if source_path.is_file() {
            if fs::metadata(&dest_path).is_ok() {
                let source_mtime = get_modified_time(&source_path)?;
                let dest_mtime = get_modified_time(&dest_path)?;

                if source_mtime > dest_mtime {
                    all_files_up_to_date = false;
                    break;
                }
            } else {
                all_files_up_to_date = false;
                break;
            }
        } else if source_path.is_dir() {
            if !dest_path.exists() {
                all_files_up_to_date = false;
                break;
            }

            if !check_files_up_to_date(&source_path, &dest_path)? {
                all_files_up_to_date = false;
                break;
            }
        }
    }

    Ok(all_files_up_to_date)
}

fn get_modified_time(path: &Path) -> io::Result<SystemTime> {
    fs::metadata(path)?.modified()
}

fn load_gitignore(config: &crate::config::Config, base_dir: &Path) -> Result<Gitignore, Box<dyn std::error::Error>> {
    let mut builder = GitignoreBuilder::new(base_dir);

    for pattern in &config.getignore.patterns {
        builder.add_line(None, pattern)?;
    }

    let gitignore = builder.build()?;
    Ok(gitignore)
}

fn should_ignore(path: &Path, gitignore: &Gitignore) -> bool {
    gitignore.matched(path, path.is_dir()).is_ignore()
}

async fn copy_recursively(
    locale: &Locale,
    source: &Path,
    destination: &Path,
    progress: &ProgressBar,
    gitignore: &Gitignore
) -> io::Result<()> {
    if should_ignore(source, gitignore) {
        return Ok(());
    }

    if source.is_dir() {
        if !destination.exists() && !should_ignore(source, gitignore) {
            fs::create_dir_all(destination)?;
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let dest_path = destination.join(entry.file_name());

            if should_ignore(&source_path, gitignore) {
                continue;
            }

            if source_path.is_dir() {
                // Рекурсивный вызов должен быть обернут в Box::pin для фиксированного размера
                Pin::from(Box::pin(copy_recursively(
                    locale, 
                    &source_path, 
                    &dest_path, 
                    progress, 
                    gitignore
                ))).await?;
            } else {
                copy_file_with_verification(locale, &source_path, &dest_path)?;
            }
            progress.inc(1);
        }
    } else {
        copy_file_with_verification(locale, source, destination)?;
        progress.inc(1);
    }
    Ok(())
}


fn copy_file_with_verification(locale: &Locale, source: &Path, destination: &Path) -> io::Result<()> {
    if !destination.exists() {
        println!("{} {:?}", locale.get("settings", "sync_file_does_not_exist"), destination);
        fs::copy(source, destination)?;
    } else if is_file_corrupted(locale, source, destination)? {
        println!("{} {:?}", locale.get("settings", "sync_file_damaged"), destination);
        fs::copy(source, destination)?;
    } else {
        println!("{} {:?}", locale.get("settings", "sync_file_relevant"), destination);
    }
    Ok(())
}


fn is_file_corrupted(locale: &Locale, source: &Path, destination: &Path) -> io::Result<bool> {
    if !destination.exists() {
        println!("{} {:?}", locale.get("settings", "sync_file_not_exist"), destination);
        return Ok(true);
    }
    let source_metadata = fs::metadata(source);
    let dest_metadata = fs::metadata(destination);

    match (source_metadata, dest_metadata) {
        (Ok(source_meta), Ok(dest_meta)) => {
            let source_size = source_meta.len();
            let dest_size = dest_meta.len();
            println!(
                "{} {:?}, {} {:?}", 
                locale.get("sync", "sync_file_size_original"), 
                source_size, 
                locale.get("sync", "sync_file_size_targeted"), 
                dest_size
            );
            if source_size != dest_size {
                println!(
                    "{} {:?}, {} {:?}", 
                    locale.get("sync", "sync_size_file_mismatch_s"), 
                    source_size, 
                    locale.get("sync", "sync_size_file_mismatch_d"), 
                    dest_size
                );
                return Ok(true);
            }

            let source_mtime = source_meta.modified()?;
            let dest_mtime = dest_meta.modified()?;
            println!(
                "{} {:?}, {} {:?}", 
                locale.get("sync", "sync_data_file_mismatch_s"), 
                source_mtime, 
                locale.get("sync", "sync_data_file_mismatch_d"), 
                dest_mtime
            );
            if source_mtime != dest_mtime {
                println!(
                    "{} {:?}, {} {:?}",  
                    locale.get("sync", "sync_data_edit_file_mismatch_s"), 
                    source_mtime, 
                    locale.get("sync", "sync_size_file_mismatch_d"), 
                    dest_mtime
                );
                return Ok(true);
            }
        },
        (Err(e), _) => {
            println!("{} {:?}", locale.get("sync", "sync_error_meta_original"), e);
            return Ok(true);
        },
        (_, Err(e)) => {
            println!("{} {:?}", locale.get("sync", "sync_error_meta_targeted"), e);
            return Ok(true);
        }
    }
    Ok(false)
}

fn count_files(path: &Path) -> io::Result<u64> {
    let mut count = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                count += count_files(&entry_path)?;
            } else {
                count += 1;
            }
        }
    } else {
        count += 1;
    }
    Ok(count)
}
