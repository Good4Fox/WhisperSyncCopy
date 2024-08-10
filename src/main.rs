use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    // Получаем путь к исполняемому файлу
    let executable_path = env::current_exe()?;
    let config_file_path = executable_path.parent().unwrap().join("WhisperSyncCopy.txt");

    // Запрашиваем язык и сохраняем его
    let language = prompt_for_language(&config_file_path)?;

    // Путь к файлу .getignore
    let getignore_path = executable_path.parent().unwrap().join(".getignore");

    // Проверка наличия файла и запрос на создание, если его нет
    if !getignore_path.exists() {
        if language == "en" {
            println!("The .getignore file doesn't exist. Would you like to create it? (y/n)");
        } else {
            println!(".getignore файл не существует. Хотите его создать? (д/н)");
        }

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        if choice.trim().to_lowercase() == "y" || choice.trim().to_lowercase() == "д" {
            File::create(&getignore_path)?;
            if language == "en" {
                println!(".getignore file created.");
            } else {
                println!(".getignore файл создан.");
            }
        }
    }

    // Запрашиваем у пользователя пути к исходной и целевой директориям
    let home_dir = prompt_for_directory("Enter the source directory path", "Введите путь к исходной директории", &config_file_path, "source", &language)?;
    let res_dir = prompt_for_directory("Enter the destination directory path", "Введите путь к целевой директории", &config_file_path, "destination", &language)?;

    // Предлагаем сохранить пути после того, как оба пути были введены
    if language == "en" {
        println!("Would you like to save these paths for future use? (y/n)");
    } else {
        println!("Хотите сохранить эти пути для дальнейшего использования? (д/н)");
    }
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    if choice.trim().to_lowercase() == "y" || choice.trim().to_lowercase() == "д" {
        save_path(&config_file_path, "source", &home_dir)?;
        save_path(&config_file_path, "destination", &res_dir)?;
        if language == "en" {
            println!("Paths saved.");
        } else {
            println!("Пути сохранены.");
        }
    }

    // Читаем игнорируемые файлы из .getignore
    let ignored_files = load_ignored_files(&getignore_path)?;

    // Копирование файлов и папок, которые были изменены
    copy_modified_files(&home_dir, &res_dir, &ignored_files, &language)?;

    if language == "en" {
        println!("Files copied successfully.");
    } else {
        println!("Файлы успешно скопированы.");
    }
    Ok(())
}

fn prompt_for_language(config_file: &Path) -> io::Result<String> {
    if let Some(saved_language) = load_saved_path(config_file, "language")? {
        return Ok(saved_language);
    }

    println!("Select language / Выберите язык:");
    println!("1: English");
    println!("2: Русский");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let language = match choice.trim() {
        "1" => "en",
        "2" => "ru",
        _ => "en", // По умолчанию английский
    };

    save_path(config_file, "language", &PathBuf::from(language))?;
    Ok(language.to_string())
}

fn prompt_for_directory(prompt_en: &str, prompt_ru: &str, config_file: &Path, key: &str, language: &str) -> io::Result<PathBuf> {
    let prompt = if language == "en" { prompt_en } else { prompt_ru };

    // Проверяем, есть ли сохраненный путь
    if let Some(saved_path) = load_saved_path(config_file, key)? {
        if language == "en" {
            println!("Found saved {} directory path: {}", key, saved_path);
            println!("Would you like to use it? (y/n)");
        } else {
            println!("Найден сохраненный путь для директории {}: {}", key, saved_path);
            println!("Хотите использовать его? (д/н)");
        }

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        if choice.trim().to_lowercase() == "y" || choice.trim().to_lowercase() == "д" {
            return Ok(PathBuf::from(saved_path));
        }
    }

    // Запрашиваем у пользователя новый путь
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(PathBuf::from(input.trim()))
}

fn load_saved_path(config_file: &Path, key: &str) -> io::Result<Option<String>> {
    if config_file.exists() {
        let file = fs::File::open(config_file)?;
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 && parts[0].trim() == key {
                    return Ok(Some(parts[1].trim().to_string()));
                }
            }
        }
    }
    Ok(None)
}

fn save_path(config_file: &Path, key: &str, value: &PathBuf) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(config_file)?;

    writeln!(file, "{}={}", key, value.to_string_lossy())?;
    Ok(())
}

fn load_ignored_files(path: &Path) -> io::Result<HashSet<String>> {
    let mut ignored_files = HashSet::new();

    if path.exists() {
        let file = fs::File::open(path)?;
        for line in io::BufReader::new(file).lines() {
            if let Ok(file_name) = line {
                ignored_files.insert(file_name.trim().to_string());
            }
        }
    }

    Ok(ignored_files)
}

fn copy_modified_files(source: &Path, destination: &Path, ignored_files: &HashSet<String>, language: &str) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        // Игнорируем файлы, указанные в .getignore
        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            if ignored_files.contains(file_name) {
                if language == "en" {
                    println!("Ignored: {:?}", path);
                } else {
                    println!("Игнорируется: {:?}", path);
                }
                continue;
            }
        }

        // Копируем файл или папку
        if let Ok(metadata) = fs::metadata(&path) {
            if metadata.is_file() {
                let dest_path = destination.join(path.strip_prefix(source).unwrap());
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&path, &dest_path)?;
                if language == "en" {
                    println!("Copied file: {:?}", path);
                } else {
                    println!("Скопирован файл: {:?}", path);
                }
            } else if metadata.is_dir() {
                let dest_path = destination.join(path.strip_prefix(source).unwrap());
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                copy_modified_files(&path, &dest_path, ignored_files, language)?; // Рекурсивно копируем папку
                if language == "en" {
                    println!("Copied directory: {:?}", path);
                } else {
                    println!("Скопирована директория: {:?}", path);
                }
            }
        }
    }
    Ok(())
}
