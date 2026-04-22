// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod downloader;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Déterminer le chemin pour le modèle
            // On peut le stocker dans app_data_dir() pour être persistant
            let app_data_dir = app.path().app_data_dir().expect("Impossible de trouver le dossier app data");
            let model_path = app_data_dir.join("u2net_background_removal_quantized.onnx");
            let model_path_str = model_path.to_string_lossy().to_string();

            let app_handle = app.handle().clone();

            // Lancer le téléchargement dans un thread séparé pour ne pas bloquer le setup
            std::thread::spawn(move || {
                match downloader::ensure_model_downloaded(&app_handle, &model_path_str) {
                    Ok(_) => println!("Modèle ONNX prêt à l'emploi : {}", model_path_str),
                    Err(e) => eprintln!("Erreur lors du téléchargement du modèle : {}", e),
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
