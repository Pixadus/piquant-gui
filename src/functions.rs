// Description: file containing public functions 
// use log::error;
use std::path::PathBuf;
use rfd::FileDialog;

/// Check to see if a file exists
pub fn get_file(file: PathBuf) -> Option<PathBuf> {
    // Check if file exists
    let path = PathBuf::from(file);
    if path.is_file() {
        Some(path)
    } else {
        None
    }
}

/// Open a file dialog to pick a file
pub fn open_fd() -> Option<PathBuf> {
    let f = FileDialog::new()
        .pick_file();

    return f
}