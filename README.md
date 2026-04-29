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
1. ```mirror_plugin``` - зеркальный разворот изображения, параметры, которые принимаются в файле с настройками плагина:
```json
{
  "vertical": true,
  "horizontal": true
}
```
```vertical``` - вертикальное отражение;
```horizontal``` - горизонтальное отражение

2. ```blur_plugin``` - размытие изображения, параметры, которые принимаются в файле с настройками плагина:
```json
{
  "radius": 3,
  "iterations": 3
}
```
```radius``` - радиус размытия;
```iterations``` - количество итераций

## Настройка окружения
- Сначала собираем плагины, полученные библиотеки плагинов по умолчанию находятся в ```/target/release```
```shell
 cargo build_mirror
```

```shell
 cargo build_blur
```

- Создаем входную директорию с файлом для обработки (расширение ```.png```)
- Создаем выходную директорию
- Создаем файл с настройками для плагина (формат ```json```)

## Примеры запуска
```shell
 cargo img_proc -i ./test_files/input/borsch.png -o ./test_files/output/new_img.png -p mirror_plugin -P ./test_files/mirror_plugin_params.json -d ./target/release
```

```shell
 cargo img_proc -i ./test_files/input/borsch.png -o ./test_files/output/new_img.png -p blur_plugin -P ./test_files/blur_plugin_params.json -d ./target/release
```