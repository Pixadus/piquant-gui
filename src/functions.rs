// Description: file containing public functions 
use log::error;
use std::path::PathBuf;

/// Check to see if a file exists and has an appropriate extension
pub fn get_file(file: PathBuf) -> Option<PathBuf> {
    // Check if file exists
    let path = PathBuf::from(file);
    if path.is_file() {
        Some(path)
    } else {
        None
    }
}