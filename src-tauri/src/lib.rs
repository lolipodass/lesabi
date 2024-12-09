use method::hide;

mod method;

#[tauri::command]
async fn encrypt(filepath: String, message: String, bits_per_channel: u8) {
    let img = image::open(filepath).unwrap();
    let mut bits_per_channel = bits_per_channel;
    if bits_per_channel > 8 {
        bits_per_channel = 8;
    }
    let result = hide(img, message.as_bytes(), bits_per_channel);
    result.save("test").unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![encrypt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
