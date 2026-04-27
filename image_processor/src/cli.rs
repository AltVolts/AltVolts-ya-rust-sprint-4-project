use clap::Parser;
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
    pub fn get_args() -> Self {
        let args = Self::parse();

        if !args.input.exists() {
            eprintln!(
                "Ошибка: входной файл не существует: {}",
                args.input.display()
            );
            std::process::exit(1);
        }
        if !args.params.exists() {
            eprintln!(
                "Ошибка: файл параметров не существует: {}",
                args.params.display()
            );
            std::process::exit(1);
        }
        if args.output.is_dir() {
            eprintln!(
                "Ошибка: выходной путь является директорией, а должен быть файлом: {}",
                args.output.display()
            );
            std::process::exit(1);
        }
        if !args.plugin_path.exists() {
            eprintln!(
                "Ошибка: директория с плагинами не найдена: {}",
                args.plugin_path.display()
            );
            std::process::exit(1);
        }
        if !args.plugin_path.is_dir() {
            eprintln!(
                "Ошибка: путь к плагину должен быть директорией: {}",
                args.plugin_path.display()
            );
            std::process::exit(1);
        }

        println!("Полученные аргументы: {:#?}", args);
        args
    }
}
