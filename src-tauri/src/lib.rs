use std::path::PathBuf;

use method::hide;

mod method;

#[tauri::command]
async fn hide_data(filepath: String, message: String, bits_per_channel: u8) -> String {
    let img = image::open(filepath).unwrap();

    let mut bits_per_channel = bits_per_channel;
    if bits_per_channel > 8 {
        bits_per_channel = 8;
    }
    let result = hide(img, message.as_bytes(), bits_per_channel);

    let buff_path = std::env::temp_dir().join("buff.png");
    result.save(&buff_path).expect("Saved is failed");
    buff_path.to_str().unwrap().to_string()
}

#[tauri::command]
async fn extract_data(filepath: String, bits_per_channel: u8) -> String {
    let img = image::open(filepath).unwrap();

    let bits_per_channel = bits_per_channel.min(8);

    let result = method::extract(img, bits_per_channel);

    String::from_utf8_lossy(&result).to_string()
}

#[tauri::command]
async fn save_file(filepath: String) {
    let buff_path = get_buff_image_path();

    std::fs::copy(buff_path, filepath).expect("Copy is failed");
}

#[tauri::command]
async fn generate_map(filepath: String) -> String {
    let img = image::open(filepath).unwrap();

    let buff_path = get_buff_image_path();

    method::image_matrix(img, buff_path.to_str().unwrap());

    buff_path.to_str().unwrap().to_string()
}

fn get_buff_image_path() -> PathBuf {
    std::env::temp_dir().join("buff.png")
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
