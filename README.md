# WhisperSyncCopy

<details>
   <summary>English</summary>

   **WhisperSyncCopy** is a tool for manually synchronizing files between two directories, copying only the changed files.

   #### Steps to use WhisperSyncCopy:

   1. **Language selection:**
      - On the first run, the program will prompt you to select a language. Use the up and down arrows to select the language, then press Enter.

   2. **Main Menu:**
      - After selecting the language, you will enter the main menu where you can choose one of the following options:
        - **Synchronize**: to start the file synchronization process.
        - **Settings**: to change the language, directories, and other parameters.
        - **Exit**: to exit the program.

   3. **Setting up source and target directories:**
      - In the settings section, the program will ask you for the paths to the source and target directories. If you have used the program before, it will apply the saved paths.

   4. **Working with the `WhisperSyncCopy.toml` file:**
      - By default, the file looks like this:
      ```
      [language]
      current = ""

      [paths]
      source = ""
      destination = ""
      save_paths = true

      [getignore]
      patterns = []

      ```
      - You can manually change the language by setting `current` to `ru` or `en`.
      - You can also change the source and destination paths for synchronization.
      - You can add ignored paths in the `patterns` section.

      **The structure will look like this:**
      ```
      [getignore]
      patterns = [
         "/*", (to block all content)
         "/*.log", (to block specific files or files in a path)
      ]
      ```

      **Example:**
      ```
      "/home/*",
      "/home/logs/*", (to block all content)
      "/home/logs", (to block folder creation)
      "/home/logs/*.log" (to block files with the .log extension)
      ```

   5. **Manual file synchronization:**
      - The program will copy all changed files from the source directory to the target directory, skipping the files specified in the `WhisperSyncCopy.toml` file.
      - **Note:** Synchronization is not automatic. You will need to manually run the program each time you want to synchronize the directories.
      - Automatic synchronization will request permission after each synchronization. This can be useful if you need to synchronize data frequently.

   6. **Settings:**
      - Change language
      - Change directory
      - Delete config

   7. **Completion:**
      - After synchronization is complete, the program will notify you that the files have been successfully copied.

</details>

<details>
   <summary>Русский</summary>

   **WhisperSyncCopy** — это инструмент для ручной синхронизации файлов между двумя директориями, который копирует только измененные файлы.

   #### Шаги использования WhisperSyncCopy:

   1. **Выбор языка:**
      - При первом запуске программа предложит вам выбрать язык. Используйте стрелки вверх и вниз для выбора языка, затем нажмите Enter.

   2. **Главное меню:**
      - После выбора языка вы попадете в главное меню, где можно выбрать один из следующих пунктов:
        - **Синхронизация**: для начала процесса синхронизации файлов.
        - **Настройки**: для изменения языка, директорий и других параметров.
        - **Выйти**: для выхода из программы.

   3. **Настройка исходной и целевой директорий:**
      - В разделе настроек программа запросит у вас пути к исходной и целевой директориям. Если вы уже использовали программу ранее, она применет сохраненые пути.

   4. **Работа с файлом `WhisperSyncCopy.toml`:**
      - По умолчанию файл выглядит так:
      ```
      [language]
      current = ""

      [paths]
      source = ""
      destination = ""
      save_paths = true

      [getignore]
      patterns = []

      ```
      - В нем можно вручную изменить язык, установив `current` на `ru` или `en`.
      - Также можно изменить исходный и конечный путь синхронизации.
      - Также можно добавить игнорируемые пути в `patterns`.

      **Структура будет выглядеть так:**
      ```
      [getignore]
      patterns = [
         "/*", (для запрета содержимого)
         "/*.log", (для запрета файла или файлов в пути)
      ]
      ```

      **Пример:**
      ```
      "/home/*",
      "/home/logs/*", (для запрета содержимого)
      "/home/logs", (для запрета создания папки)
      "/home/logs/*.log" (для запрета файла с расширением .log)
      ```

   5. **Ручная синхронизация файлов:**
      - Программа скопирует все измененные файлы из исходной директории в целевую, пропуская те файлы, которые указаны в файле `WhisperSyncCopy.toml`.
      - **Примечание:** Синхронизация не является автоматической. Вам нужно будет вручную запускать программу каждый раз, когда вы хотите синхронизировать директории.
      - Автоматическая синхронизация после каждой синхронизации будет запрашивать разрешение на синхронизацию. Она будет полезной в случае, если нужно часто синхронизировать данные.

   6. **Настройки:**
      - Изменить язык
      - Изменить директорию
      - Удалить конфиг

   7. **Завершение:**
      - После завершения синхронизации программа уведомит вас о том, что файлы успешно скопированы.

</details>
