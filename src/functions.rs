// Description: file containing public functions 
use log::error;
use std::path::PathBuf;
use image::ImageFormat;

/// Get a list of all files within a given directory
pub fn get_files_around(file: &PathBuf) -> Option<Vec<PathBuf>> {
    // Only return top-level files
    let mut files = Vec::<PathBuf>::new();
    for file in file.read_dir().expect("Unable to read directory") {
        if let Ok(file) = file {
            files.push(file.path());
        }
    }
    if !files.is_empty() {
        Some(files)
    } else {
        None
    }
}

/// Check to see if a file exists and has an appropriate extension
pub fn get_file(file: PathBuf) -> Option<PathBuf> {
    // Check if file exists
    let path = PathBuf::from(file);
    if path.is_file() {
        // Check if appropriate extension
        match ImageFormat::from_path(&path) {
            Ok(fmt) => { 
                println!("{:?}", fmt); 
                Some(path)
            },
            Err(e) => { 
                error!("{}.", e);
                None
            }
        }
    } else {
        None
    }
}