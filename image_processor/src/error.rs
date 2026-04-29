use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImgError {
    #[error("Ошибка ввода-вывода: {0}")]
    Io(#[from] std::io::Error),

    #[error("Ошибка загрузки плагина: {0}")]
    Plugin(#[from] libloading::Error),

    #[error("Ошибка работы с изображением: {0}")]
    Image(#[from] image::ImageError),

    #[error("Входной файл не найден: {0}")]
    InputFileNotFound(PathBuf),

    #[error("Файл параметров не найден: {0}")]
    ParamsFileNotFound(PathBuf),

    #[error("Путь сохранения ведёт к директории, а не к файлу: {0}")]
    OutputIsDirectory(PathBuf),

    #[error("Директория плагинов не найдена: {0}")]
    PluginDirNotFound(PathBuf),

    #[error("Путь к плагинам не является директорией: {0}")]
    PluginDirNotDirectory(PathBuf),

    #[error("Некорректный размер буфера изображения: ожидается {expected}, получено {actual}")]
    BufferSizeMismatch { expected: usize, actual: usize },

    #[error("Строка параметров содержит нулевой байт")]
    ParamContainsNul(#[from] std::ffi::NulError),

    #[error("{0}")]
    Other(String),
}

impl ImgError {
    pub fn input_not_found(path: impl Into<PathBuf>) -> Self {
        ImgError::InputFileNotFound(path.into())
    }

    pub fn params_not_found(path: impl Into<PathBuf>) -> Self {
        ImgError::ParamsFileNotFound(path.into())
    }

    pub fn output_is_directory(path: impl Into<PathBuf>) -> Self {
        ImgError::OutputIsDirectory(path.into())
    }

    pub fn plugin_dir_not_found(path: impl Into<PathBuf>) -> Self {
        ImgError::PluginDirNotFound(path.into())
    }

    pub fn plugin_dir_not_directory(path: impl Into<PathBuf>) -> Self {
        ImgError::PluginDirNotDirectory(path.into())
    }
}
