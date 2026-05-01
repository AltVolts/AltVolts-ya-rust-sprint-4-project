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
        "{}mirror_plugin{}",
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
        "Plugin library not found at {}\nBuild it with: cargo build -p mirror_plugin",
        lib_path.display()
    );
    let lib = unsafe { Library::new(&lib_path).expect("Failed to load mirror_plugin") };
    let lib = Box::leak(Box::new(lib));
    unsafe { lib.get(b"process_image\0").expect("Symbol process_image not found") }
}

/// Создаёт тестовое RGBA-изображение 2x2.
/// Пиксели (R, G, B, A):
/// (1,0,0,255), (2,0,0,255)
/// (3,0,0,255), (4,0,0,255)
fn test_image_2x2() -> Vec<u8> {
    vec![
        1, 0, 0, 255,   // пиксель (0,0)
        2, 0, 0, 255,   // пиксель (1,0)
        3, 0, 0, 255,   // пиксель (0,1)
        4, 0, 0, 255,   // пиксель (1,1)
    ]
}

#[test]
fn mirror_null_params() {
    let mut data = test_image_2x2();
    let original = data.clone();
    let process_image = load_plugin();
    // Передаём null-указатель – должны использоваться настройки по умолчанию (ничего не делать)
    unsafe { process_image(2, 2, data.as_mut_ptr(), std::ptr::null()) };
    assert_eq!(data, original);
}

#[test]
fn mirror_empty_json_defaults() {
    let mut data = test_image_2x2();
    let original = data.clone();
    let params = CString::new("{}").unwrap();
    let process_image = load_plugin();
    unsafe { process_image(2, 2, data.as_mut_ptr(), params.as_ptr()) };
    assert_eq!(data, original);
}

#[test]
fn mirror_null_data_does_not_crash() {
    let params = CString::new("{}").unwrap();
    let process_image = load_plugin();
    // Передаём nullptr в rgba_data – плагин должен просто выйти без паники.
    unsafe { process_image(2, 2, std::ptr::null_mut(), params.as_ptr()) };
}