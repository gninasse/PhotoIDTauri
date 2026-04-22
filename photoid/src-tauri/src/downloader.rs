use anyhow::Context;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tauri::{AppHandle, Emitter, Runtime};

const MODEL_URL: &str = "https://github.com/gninasse/ai_models/raw/refs/heads/main/image/u2net_background_removal_quantized.onnx";

#[derive(Clone, Serialize)]
struct DownloadProgress {
    downloaded: u64,
    total: u64,
}

pub fn ensure_model_downloaded<R: Runtime>(
    app: &AppHandle<R>,
    model_path: &str,
) -> Result<(), anyhow::Error> {
    let path = Path::new(model_path);
    if path.exists() {
        // Le modèle existe déjà
        return Ok(());
    }

    // Assurez-vous que le dossier parent existe
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut response = reqwest::blocking::get(MODEL_URL)
        .context("Échec de la connexion pour télécharger le modèle ONNX")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Échec du téléchargement: statut HTTP {}",
            response.status()
        ));
    }

    let total_size = response
        .content_length()
        .unwrap_or(0); // 0 si la taille est inconnue

    let mut file = File::create(path).context("Impossible de créer le fichier de modèle local")?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = std::io::Read::read(&mut response, &mut buffer)?;
        if bytes_read == 0 {
            break; // Fin du flux
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        // Émettre la progression
        let _ = app.emit(
            "download-progress",
            DownloadProgress {
                downloaded,
                total: total_size,
            },
        );
    }

    Ok(())
}
