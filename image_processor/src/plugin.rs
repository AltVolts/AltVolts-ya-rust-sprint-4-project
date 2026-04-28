use core::ffi::{c_char};
use std::ffi::CString;
use libloading::{Library, Symbol};
use std::path::Path;

const RGB_PIXEL_BYTES: u32 = 4;

/// Тип функции плагина (C ABI)
pub type ProcessImageFn = unsafe extern "C" fn(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
);

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
    pub fn new(name: &str, plugin_dir: &Path) -> Result<Self, libloading::Error> {
        let lib_filename = match std::env::consts::OS {
            "windows" => format!("{}.dll", name),
            "macos" => format!("lib{}.dylib", name),
            _ => format!("lib{}.so", name),
        };
        let lib_path = plugin_dir.join(lib_filename);

        unsafe {
            let lib = Library::new(lib_path)?;
            Ok(Plugin { _lib: lib })
        }
    }

    /// Возвращает интерфейс для вызова функции плагина.
    /// Функция ищется по имени "process_image" (как в C ABI).
    pub fn interface(&self) -> Result<PluginInterface<'_>, Box<dyn std::error::Error>> {
        Ok(PluginInterface {
            process_image: unsafe { self._lib.get(b"process_image\0") }?,
        })
    }

    pub fn process_image(
        &self,
        width: u32,
        height: u32,
        rgba_data: &mut [u8],
        params: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expected_len = (width * height * RGB_PIXEL_BYTES) as usize;
        if rgba_data.len() != expected_len {
            return Err(format!("buffer size mismatch: expected {}, got {}", expected_len, rgba_data.len()).into());
        }
        let iface = self.interface()?;
        let params_cstr = CString::new(params)?;
        unsafe {
            (iface.process_image)(width, height, rgba_data.as_mut_ptr(), params_cstr.as_ptr());
        }
        Ok(())
    }
}
