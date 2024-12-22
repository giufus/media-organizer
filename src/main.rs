use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use std::env;
use walkdir::WalkDir;
use regex::Regex;
use chrono::NaiveDateTime;

fn main() -> std::io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if start folder is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <start_folder>", args[0]);
        std::process::exit(1);
    }

    // Get the start directory from command line argument
    let start_dir = PathBuf::from(&args[1]);

    // Verify the directory exists
    if !start_dir.exists() || !start_dir.is_dir() {
        eprintln!("Error: '{}' is not a valid directory", start_dir.display());
        std::process::exit(1);
    }

    println!("Starting media organization from: {}", start_dir.display());
    organize_media_files(&start_dir)?;
    println!("Organization complete!");
    Ok(())
}

fn organize_media_files(root_dir: &Path) -> std::io::Result<()> {
    // Define regex patterns for date extraction
    // Matches patterns like: 20231225, 2023-12-25, 2023_12_25
    let date_pattern = Regex::new(r"(?i)(20\d{2})[-_]?(\d{2})[-_]?(\d{2})").unwrap();

    // Define supported media extensions
    let image_extensions: Vec<&str> = vec!["jpg", "jpeg", "png", "gif", "bmp", "webp"];
    let video_extensions: Vec<&str> = vec!["mp4", "mov", "avi", "mkv", "wmv", "flv"];

    // Walk through all files in the directory
    for entry in WalkDir::new(root_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip if not a file
        if !path.is_file() {
            continue;
        }

        // Get file extension
        let extension = match path.extension() {
            Some(ext) => ext.to_string_lossy().to_lowercase(),
            None => continue,
        };

        // Check if it's a media file
        let media_type = if image_extensions.contains(&extension.as_str()) {
            Some("images")
        } else if video_extensions.contains(&extension.as_str()) {
            Some("videos")
        } else {
            None
        };

        // Process only media files
        if let Some(media_folder) = media_type {
            let file_name = path.file_name().unwrap().to_string_lossy();

            // Try to extract year from filename
            if let Some(captures) = date_pattern.captures(&file_name) {
                let year = &captures[1];

                // Create year directory under media type if it doesn't exist
                let target_dir = root_dir.join(media_folder).join(year);
                create_dir_all(&target_dir)?;

                // Create the target path
                let target_path = target_dir.join(path.file_name().unwrap());

                // Move the file
                if path != target_path {
                    println!("Moving {:?} to {:?}", path, target_path);
                    fs::rename(path, target_path)?;
                }
            } else {
                println!("Could not extract date from filename: {:?}", file_name);
            }
        }
    }

    Ok(())
}