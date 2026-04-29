use clap::Parser;
use image_plugin::error::ImgError;
use log::{debug, warn};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "image_processor", about = "CLI для обработки изображений", long_about = None)]
pub(crate) struct Cli {
    /// Путь к исходному PNG-изображению
    #[arg(short, long, help = "путь к исходному PNG-изображению")]
    pub input: PathBuf,

    /// Путь для сохранения обработанного изображения
    #[arg(
        short,
        long,
        help = "путь, по которому будет сохранено обработанное изображение"
    )]
    pub output: PathBuf,

    /// Имя плагина без расширения
    #[arg(
        short = 'p',
        long,
        help = "имя плагина (без расширения, например invert)"
    )]
    pub plugin: String,

    /// Путь к файлу с параметрами
    #[arg(
        short = 'P',
        long,
        help = "путь к текстовому файлу с параметрами обработки"
    )]
    pub params: PathBuf,

    /// Путь к директории с плагинами
    #[arg(
        short = 'd',
        long,
        default_value = "target/debug",
        help = "путь к директории, где находится плагин"
    )]
    pub plugin_path: PathBuf,
}

impl Cli {
    pub fn get_args() -> Result<Self, ImgError> {
        let args = Self::parse();

        if !args.input.exists() {
            return Err(ImgError::input_not_found(args.input));
        }

        if !args.params.exists() {
            return Err(ImgError::params_not_found(args.params));
        }

        if args.output.exists() && args.output.is_dir() {
            return Err(ImgError::output_is_directory(args.output));
        }

        if let Some(parent) = args.output.parent()
            && !parent.exists()
        {
            warn!(
                "Родительская директория не найдена, создаём: {}",
                parent.display()
            );
            fs::create_dir_all(parent)?;
        }

        if !args.plugin_path.exists() {
            return Err(ImgError::plugin_dir_not_found(args.plugin_path));
        }
        if !args.plugin_path.is_dir() {
            return Err(ImgError::plugin_dir_not_directory(args.plugin_path));
        }

        debug!("Полученные аргументы: {:#?}", args);
        Ok(args)
    }
}
