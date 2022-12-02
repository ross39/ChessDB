#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod pgn;

fn  main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pgn::create_pgn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
