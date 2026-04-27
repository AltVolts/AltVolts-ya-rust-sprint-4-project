use std::path::PathBuf;
use clap::{Parser};


#[derive(Parser, Debug)]
#[command(name = "image_processor", about = "CLI для обработки изображений", long_about = None)]
pub(crate) struct Cli {
    /// Путь к исходному PNG-изображению
    #[arg(short, long, help = "путь к исходному PNG-изображению")]
    pub input: PathBuf,

    /// Путь для сохранения обработанного изображения
    #[arg(short, long, help = "путь, по которому будет сохранено обработанное изображение")]
    pub output: PathBuf,

    /// Имя плагина без расширения
    #[arg(short, long, help = "имя плагина (без расширения, например invert)")]
    pub plugin: String,

    /// Путь к файлу с параметрами
    #[arg(short, long, help = "путь к текстовому файлу с параметрами обработки")]
    pub params: PathBuf,

    /// Путь к директории с плагинами
    #[arg(short, long, default_value = "target/debug", help = "путь к директории, где находится плагин")]
    pub plugin_path: PathBuf,
}