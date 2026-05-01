use libloading::{Library, Symbol};
use std::ffi::CString;
use std::path::PathBuf;

type ProcessImageFn =
    unsafe extern "C" fn(width: u32, height: u32, rgba_data: *mut u8, params: *const std::ffi::c_char);

/// Возвращает путь к динамической библиотеке
fn plugin_lib_path() -> PathBuf {
    let exe = std::env::current_exe().expect("Cannot get test executable path");
    let deps_dir = exe.parent().expect("Executable must be in a directory (deps)");
    // deps_dir — target/debug/deps (или target/release/deps)
    let target_profile_dir = deps_dir.parent().expect("deps/ must have a parent (target/<profile>)");

    let lib_name = format!(
        "{}blur_plugin{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    target_profile_dir.join(lib_name)
}

/// Загружает библиотеку и возвращает 'static символ.
fn load_plugin() -> Symbol<'static, ProcessImageFn> {
    let lib_path = plugin_lib_path();
    assert!(
        lib_path.exists(),
        "Plugin library not found at {}\nBuild it with: cargo build -p blur_plugin",
        lib_path.display()
    );
    let lib = unsafe { Library::new(&lib_path).expect("Failed to load blur_plugin") };
    let lib = Box::leak(Box::new(lib));
    unsafe { lib.get(b"process_image\0").expect("Symbol process_image not found") }
}

/// Создаёт однородное RGBA-изображение заданного размера с указанным цветом.
fn solid_rgba(r: u8, g: u8, b: u8, a: u8, pixels: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(pixels * 4);
    for _ in 0..pixels {
        v.extend_from_slice(&[r, g, b, a]);
    }
    v
}

#[test]
fn blur_null_data_does_not_crash() {
    // Передаём nullptr в rgba_data – плагин должен просто выйти без паники
    let params = CString::new(r#"{ "radius": 3 }"#).unwrap();
    let process_image = load_plugin();
    unsafe { process_image(3, 3, std::ptr::null_mut(), params.as_ptr()) };
}

#[test]
fn blur_null_params_uses_defaults() {
    // Передаём null-указатель параметров – должны примениться умолчания (radius=0 → ничего не делать)
    let w = 3; let h = 3;
    let mut data = solid_rgba(5, 5, 5, 255, 9);
    let original = data.clone();
    let process_image = load_plugin();
    unsafe { process_image(w, h, data.as_mut_ptr(), std::ptr::null()) };
    assert_eq!(data, original);
}