use serde::Deserialize;
use std::ffi::{CStr, c_char};

const RGB_PIXEL_BYTES: usize = 4;

#[derive(Deserialize, Debug)]
struct PluginParams {
    #[serde(default)]
    radius: u32,
    #[serde(default = "default_iterations")]
    iterations: u32,
}

fn default_iterations() -> u32 {
    1
}

#[unsafe(no_mangle)]
extern "C" fn process_image(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char) {
    let params_str = if params.is_null() {
        "{}"
    } else {
        unsafe { CStr::from_ptr(params) }.to_str().unwrap_or("{}")
    };

    let params: PluginParams = serde_json::from_str(params_str).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON: {}, using defaults", e);
        PluginParams {
            radius: 0,
            iterations: 1,
        }
    });

    if rgba_data.is_null() {
        eprintln!("Null pointer for rgba_data!");
        return;
    }

    let width = width as usize;
    let height = height as usize;
    let total_bytes = width * height * RGB_PIXEL_BYTES;
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, total_bytes) };

    if params.radius == 0 || params.iterations == 0 {
        println!("Blur disabled (radius=0 or iterations=0)");
        return;
    }

    println!(
        "Blur plugin: radius={}, iterations={} on {}x{} image",
        params.radius, params.iterations, width, height
    );

    for i in 0..params.iterations {
        blur_pass(data, width, height, params.radius);
        println!("Blur iteration {}/{} done", i + 1, params.iterations);
    }
}

fn blur_pass(data: &mut [u8], width: usize, height: usize, radius: u32) {
    let r = radius as i32;
    let diameter = (2 * r + 1) as usize;

    // --- 1. Предрассчитываем веса для всего окна ---
    let mut weights = Vec::with_capacity(diameter * diameter);
    let mut weight_sum: f64 = 0.0;
    for dy in -r..=r {
        for dx in -r..=r {
            let dist = dx.abs() + dy.abs();
            let w = (r + 1 - dist) as f64;
            weights.push(w);
            weight_sum += w;
        }
    }

    // --- 2. Копируем оригинал для чтения ---
    let original = data.to_vec();

    // --- 3. Применяем фильтр к каждому пикселю ---
    for y in 0..height {
        for x in 0..width {
            let (mut sum_r, mut sum_g, mut sum_b, mut sum_a) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);

            let mut idx = 0; // индекс в таблице весов
            for dy in -r..=r {
                let ny_unclamped = y as i32 + dy;
                // Зажимаем координату Y в границы [0, height-1]
                let ny = ny_unclamped.max(0).min(height as i32 - 1) as usize;

                for dx in -r..=r {
                    let nx_unclamped = x as i32 + dx;
                    // Зажимаем координату X в границы [0, width-1]
                    let nx = nx_unclamped.max(0).min(width as i32 - 1) as usize;

                    let w = weights[idx];
                    let src_offset = (ny * width + nx) * RGB_PIXEL_BYTES;

                    // Считываем цвета (уже зажатого) пикселя
                    let r_val = original[src_offset] as f64;
                    let g_val = original[src_offset + 1] as f64;
                    let b_val = original[src_offset + 2] as f64;
                    let a_val = original[src_offset + 3] as f64;

                    sum_r += w * r_val;
                    sum_g += w * g_val;
                    sum_b += w * b_val;
                    sum_a += w * a_val;

                    idx += 1;
                }
            }

            // --- 4. Нормируем и записываем результат ---К
            let dst_offset = (y * width + x) * RGB_PIXEL_BYTES;
            data[dst_offset] = (sum_r / weight_sum).round() as u8;
            data[dst_offset + 1] = (sum_g / weight_sum).round() as u8;
            data[dst_offset + 2] = (sum_b / weight_sum).round() as u8;
            data[dst_offset + 3] = (sum_a / weight_sum).round() as u8;
        }
    }
}
