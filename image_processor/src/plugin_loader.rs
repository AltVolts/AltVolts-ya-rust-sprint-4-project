use crate::error::ImgError;
use core::ffi::c_char;
use libloading::{Library, Symbol};
use std::ffi::CString;
use std::path::Path;

const RGB_PIXEL_BYTES: u32 = 4;

/// Тип функции плагина (C ABI)
pub type ProcessImageFn =
    unsafe extern "C" fn(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char);

/// Интерфейс плагина: хранит указатель на функцию process_image.
pub struct PluginInterface<'a> {
    pub process_image: Symbol<'a, ProcessImageFn>,
}

/// Загруженная динамическая библиотека плагина.
pub struct Plugin {
    _lib: Library,
}

impl Plugin {
    /// Загружает плагин по имени (без расширения) из указанной директории.
    pub fn new(name: &str, plugin_dir: &Path) -> Result<Self, ImgError> {
        let lib_filename = plugin_filename(name);
        let lib_path = plugin_dir.join(lib_filename);

        // SAFETY: загрузка динамической библиотеки через libloading.
        // Путь к файлу проверен (валидация в cli), так что мы не загружаем произвольный файл без контроля.
        let lib = unsafe { Library::new(lib_path)? };
        Ok(Plugin { _lib: lib })
    }

    /// Возвращает интерфейс для вызова функции плагина.
    /// Функция ищется по имени "process_image" (как в C ABI).
    pub fn interface(&self) -> Result<PluginInterface<'_>, ImgError> {
        // SAFETY: получение символа "process_image" из библиотеки.
        // Используем нуль-терминированный байтовый литерал, сигнатура соответствует
        // ожидаемому C-ABI. Libloading гарантирует, что символ либо корректен, либо вернёт ошибку.
        let process_image = unsafe { self._lib.get(b"process_image\0")? };
        Ok(PluginInterface { process_image })
    }

    /// Безопасная функция-обертка для вызова обработки изображения.
    pub fn process_image(
        &self,
        width: u32,
        height: u32,
        rgba_data: &mut [u8],
        params: &str,
    ) -> Result<(), ImgError> {
        let expected_len = (width * height * RGB_PIXEL_BYTES) as usize;
        if rgba_data.len() != expected_len {
            return Err(ImgError::BufferSizeMismatch {
                expected: expected_len,
                actual: rgba_data.len(),
            });
        }
        let iface = self.interface()?;
        let params_cstr = CString::new(params)?;

        // SAFETY: вызов C-функции плагина.
        // - Буфер rgba_data имеет корректный размер (проверено выше) и будет жить на протяжении вызова.
        // - params_cstr — валидный CString, гарантированно не nul-содержащий.
        // - Плагин обязан не выходить за границы буфера и не сохранять указатели после возврата.
        unsafe {
            (iface.process_image)(width, height, rgba_data.as_mut_ptr(), params_cstr.as_ptr());
        }
        Ok(())
    }
}

pub fn plugin_filename(name: &str) -> String {
    match std::env::consts::OS {
        "windows" => format!("{}.dll", name),
        "macos" => format!("lib{}.dylib", name),
        _ => format!("lib{}.so", name),
    }
}
