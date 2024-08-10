# WhisperSyncCopy

<details>
   <summary>English</summary>
   
   **Is still in development!**
   
   **WhisperSyncCopy** is a tool designed to manually synchronize files between two directories, ensuring that only modified files are copied.

   #### Steps to use WhisperSyncCopy:

   1. **Language Selection:**
      - On the first launch, the program will ask you to select a language. Choose "1" for English or "2" for Russian.

   2. **Configuring Source and Destination Directories:**
      - The program will prompt you to enter the source and destination directory paths. If you have used the program before, it will ask if you want to use the previously saved paths.
      - After entering the paths, you will have the option to save them for future use.

   3. **Handling the `.getignore` File:**
      - If a `.getignore` file does not exist in the program's directory, the program will ask if you want to create one. This file allows you to specify files or directories that should be ignored during synchronization.
      - To add files or directories to be ignored, simply list their names in the `.getignore` file, one per line.

      **Example:**
      - To ignore a specific file: `file.txt`
      - To ignore all files in a directory: `directory_name/`
      - To ignore a specific file inside a directory: `directory_name/file.txt`

   4. **Manual File Synchronization:**
      - The program will copy all modified files from the source to the destination directory, skipping any files listed in the `.getignore` file.
      - **Note:** Synchronization is not automatic. You will need to manually run the program each time you want to synchronize the directories.

   5. **Completion:**
      - After the synchronization is complete, the program will notify you that the files have been successfully copied.

</details>

<details>
   <summary>Русский</summary>
   
   **Все ещё в разработке!**

   **WhisperSyncCopy** — это инструмент для ручной синхронизации файлов между двумя директориями, который копирует только измененные файлы.

   #### Шаги использования WhisperSyncCopy:

   1. **Выбор языка:**
      - При первом запуске программа предложит вам выбрать язык. Нажмите "1" для английского или "2" для русского.

   2. **Настройка исходной и целевой директорий:**
      - Программа запросит у вас пути к исходной и целевой директориям. Если вы уже использовали программу ранее, она предложит использовать сохраненные пути.
      - После ввода путей вам будет предложено сохранить их для дальнейшего использования.

   3. **Работа с файлом `.getignore`:**
      - Если файл `.getignore` не существует в директории программы, программа спросит, хотите ли вы его создать. Этот файл позволяет указать файлы или директории, которые нужно игнорировать во время синхронизации.
      - Чтобы добавить игнорируемые файлы или директории, просто укажите их названия в файле `.getignore`, по одному на каждой строке.

      **Пример:**
      - Чтобы игнорировать конкретный файл: `file.txt`
      - Чтобы игнорировать все файлы в директории: `directory_name/`
      - Чтобы игнорировать конкретный файл внутри директории: `directory_name/file.txt`

   4. **Ручная синхронизация файлов:**
      - Программа скопирует все измененные файлы из исходной директории в целевую, пропуская те файлы, которые указаны в файле `.getignore`.
      - **Примечание:** Синхронизация не является автоматической. Вам нужно будет вручную запускать программу каждый раз, когда вы хотите синхронизировать директории.

   5. **Завершение:**
      - После завершения синхронизации программа уведомит вас о том, что файлы успешно скопированы.

</details>
