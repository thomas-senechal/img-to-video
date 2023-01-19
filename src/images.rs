//! Contains all functions related to images.

use image::DynamicImage;
use std::fs;
use std::io;
use std::path::Path;

/// Get all images from a given directory.
/// Images are sorted by name.
///  
/// # Arguments
/// * `path` - The path to the directory.
///
/// # Returns
/// A vector of paths to all images in the directory as [DynamicImage].
///
/// # Example
/// ```
/// use images::get_images;
/// let images = get_images("src/images");
/// ```
///
/// # Errors
/// If the directory does not exist, or if it is not a directory, or if it is empty, an error is returned.
///
pub fn get_images<P: AsRef<Path>>(path: &P) -> Result<Vec<DynamicImage>, io::Error> {
    let mut images: Vec<DynamicImage> = Vec::new();
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_by_key(|a| a.file_name());

    for entry in entries {
        let path = entry.path();
        let ext = match path.extension() {
            Some(e) => e,
            None => continue,
        };
        if ext == "jpg" || ext == "png" || ext == "jpeg" {
            let image = image::open(&path).unwrap();
            images.push(image);
            debug!("Added {}", path.display());
        }
    }
    Ok(images)
}
