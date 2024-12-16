use std::path::PathBuf;
use tauri::Emitter;

use method::hide;

mod pixel_manipulations;
mod image_matrix;
mod method;

#[tauri::command]
async fn hide_data(
    app: tauri::AppHandle,
    filepath: String,
    message: String,
    bits_per_channel: u8
) -> Result<String, String> {
    let img = image::open(filepath).map_err(|_| "Failed to open image")?;

    let bits_per_channel = bits_per_channel.min(8);

    let start_time = std::time::Instant::now();
    let result = hide(img, message.as_bytes(), bits_per_channel)?;
    let duration = start_time.elapsed();

    app.emit("hide_function_duration", duration.as_millis()).map_err(|_| "Failed to emit event")?;

    let buff_path = std::env::temp_dir().join("buff.png");

    result.save(&buff_path).map_err(|_| "Failed to save image")?;

    Ok(buff_path.to_str().unwrap().to_string())
}

#[tauri::command]
async fn extract_data(
    app: tauri::AppHandle,
    filepath: String,
    bits_per_channel: u8
) -> Result<String, String> {
    let img = image::open(filepath).map_err(|_| "Failed to open image")?;

    let bits_per_channel = bits_per_channel.min(8);

    let start_time = std::time::Instant::now();
    let result = method::extract(img, bits_per_channel)?;
    let duration = start_time.elapsed();
    app
        .emit("extract_function_duration", duration.as_millis())
        .map_err(|_| "Failed to emit event")?;

    Ok(String::from_utf8_lossy(&result).to_string())
}

#[tauri::command]
async fn save_file(filepath: String) {
    let buff_path = get_buff_image_path("buff.png");

    std::fs::copy(buff_path, filepath).expect("Copy is failed");
}

#[tauri::command]
async fn generate_map(filepath: String, name: &str) -> Result<String, String> {
    let img = image::open(filepath).map_err(|_| "Failed to open image")?;

    let buff_path = get_buff_image_path(&(name.to_string() + "matrix.png"));

    image_matrix::image_matrix(img, buff_path.to_str().unwrap());

    Ok(buff_path.to_str().expect("Failed to get buff path").to_string())
}

fn get_buff_image_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(name)
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![hide_data, extract_data, save_file, generate_map])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
