use std::path::PathBuf;
use tokio::fs;

use crate::error::Error;

/// Download a TMDB image, resize to ≤500px wide, and save as JPEG to the project's images folder.
/// Returns the absolute local path to the cached file.
/// If the file already exists it is returned immediately (permanent cache).
#[tauri::command]
pub async fn cache_image(
    project_path: String,
    url: String,
    filename: String,
    subdir: String, // "posters" or "photos"
) -> Result<String, Error> {
    let images_dir = PathBuf::from(&project_path)
        .join("images")
        .join(&subdir);

    fs::create_dir_all(&images_dir).await?;

    let out_path = images_dir.join(format!("{}.jpg", filename));

    // Already cached — skip download
    if fs::metadata(&out_path).await.is_ok() {
        return Ok(out_path.to_string_lossy().to_string());
    }

    // Download
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| Error::ImageDownload(e.to_string()))?;

    if !response.status().is_success() {
        return Err(Error::ImageDownload(format!("HTTP {}", response.status())));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| Error::ImageDownload(e.to_string()))?
        .to_vec();

    // Decode, resize, encode — CPU-bound work on a blocking thread
    let out_path_clone = out_path.clone();
    tokio::task::spawn_blocking(move || -> Result<(), Error> {
        use image::GenericImageView;

        let img = image::load_from_memory(&bytes)
            .map_err(|e| Error::InvalidImage(e.to_string()))?;

        let (w, _) = img.dimensions();
        let resized = if w > 500 {
            img.resize(500, u32::MAX, image::imageops::FilterType::Lanczos3)
        } else {
            img
        };

        let rgb = resized.to_rgb8();

        use image::codecs::jpeg::JpegEncoder;
        use std::fs::File;
        use std::io::BufWriter;

        let file =
            File::create(&out_path_clone).map_err(|e| Error::FileWrite(e.to_string()))?;
        let mut writer = BufWriter::new(file);
        let mut encoder = JpegEncoder::new_with_quality(&mut writer, 80);
        encoder
            .encode(
                rgb.as_raw(),
                rgb.width(),
                rgb.height(),
                image::ColorType::Rgb8,
            )
            .map_err(|e| Error::FileWrite(e.to_string()))?;

        Ok(())
    })
    .await
    .map_err(|e| Error::Unknown(e.to_string()))??;

    Ok(out_path.to_string_lossy().to_string())
}
