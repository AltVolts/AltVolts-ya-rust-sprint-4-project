# Проектная работа модуля 4. Обработчик изображений с плагинами
## Данный проект представляет собой CLI-приложение, которое загружает изображение, применяет к нему указанный плагин обработки и сохраняет результат.

## Архитектура
```
ya-rust-sprint-4-project/
├── Cargo.toml                  # Workspace
├── image_processor/            # Image processor package           
├── mirror_plugin/              # Mirror plugin package
├── blur_plugin/                # Blur plugin package
├── test_files/                 # Test input/output dirs and plugin param files 
└── README.md
```

## Виды плагинов
1. ```mirror_plugin``` - зеркальный разворот изображения; формат файла с настройками плагина:
```json
{
  "vertical": true,
  "horizontal": true
}
```
```vertical``` - вертикальное отражение;
```horizontal``` - горизонтальное отражение

2. ```blur_plugin``` - размытие изображения, формат файла с настройками плагина:
```json
{
  "radius": 3,
  "iterations": 3
}
```
```radius``` - радиус размытия;
```iterations``` - количество итераций

## Настройка окружения
- Сначала собираем плагины - полученные библиотеки плагинов по умолчанию будут находиться в ```/target/release```
```shell
 cargo build_mirror
```

```shell
 cargo build_blur
```

- Создаем входную директорию с файлом для обработки (расширение ```.png```)
- Создаем выходную директорию
- Создаем файл с настройками для плагина (формат ```json```)

## Описание параметров запуска программы:
```text
CLI для обработки изображений

Usage: image_processor.exe [OPTIONS] --input <INPUT> --output <OUTPUT> --plugin <PLUGIN> --params <PARAMS>

Options:
  -i, --input <INPUT>              путь к исходному PNG-изображению
  -o, --output <OUTPUT>            путь, по которому будет сохранено обработанное изображение
  -p, --plugin <PLUGIN>            имя плагина (без расширения, например invert)
  -P, --params <PARAMS>            путь к текстовому файлу с параметрами обработки
  -d, --plugin-path <PLUGIN_PATH>  путь к директории, где находится плагин [default: target/debug]
  -h, --help                       Print help
```

## Примеры запуска
```shell
 cargo img_proc -i ./test_files/input/borsch.png -o ./test_files/output/new_img.png -p mirror_plugin -P ./test_files/mirror_plugin_params.json -d ./target/release
```

```shell
 cargo img_proc -i ./test_files/input/borsch.png -o ./test_files/output/new_img.png -p blur_plugin -P ./test_files/blur_plugin_params.json -d ./target/release
```