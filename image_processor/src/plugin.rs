use core::ffi::{c_char, c_uint};
use libloading::{Library, Symbol};
use std::path::Path;

/// Интерфейс плагина: хранит указатель на функцию process_image.
pub struct PluginInterface<'a> {
    pub process_image: Symbol<
        'a,
        extern "C" fn(width: c_uint, height: c_uint, rgba_data: *mut u8, params: *const c_char),
    >,
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
        if !lib_path.exists() {
            eprintln!("Plugin not found: {}", lib_path.display());
            std::process::exit(1);
        }
        unsafe {
            let lib = Library::new(lib_path)?;
            Ok(Plugin { _lib: lib })
        }
    }

    /// Возвращает интерфейс для вызова функции плагина.
    /// Функция ищется по имени "process_image" (как в C ABI).
    pub fn interface(&self) -> Result<PluginInterface<'_>, Box<dyn std::error::Error>> {
        Ok(PluginInterface {
            process_image: unsafe { self._lib.get("process_image") }?,
        })
    }
}
