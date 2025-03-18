// Description: file containing public functions 
// use log::error;
use std::path::PathBuf;
use rfd::FileDialog;
use eframe::egui::Color32;

use crate::gui;

/// Open a file dialog to pick a file
pub fn open_fd() -> Option<PathBuf> {
    let f = FileDialog::new()
        .pick_file();
    f
}

/// Check to see if a file path is valid & exists
pub fn check_path(path: String) -> bool {
    let path = PathBuf::from(path);
    path.try_exists().unwrap()
}

/// Disable all elements in a boolean vec
pub fn set_all_disabled(bool_vec: &mut Vec<bool>){
    for b in bool_vec.iter_mut() {
        *b = false;
    }
}

/// Enable nth entry in a boolean vec
pub fn set_enabled(bool_vec: &mut Vec<bool>, indices: Vec<usize>) {
    for index in indices.iter() {
        bool_vec[*index] = true;
    }
}

/// Handle radio button pressing
pub fn handle_task_selection(sel_task: &mut gui::Tasks, bool_vec: &mut Vec<bool>, args: &mut Vec<String>) {
    // Reset enables and CLI arguments
    set_all_disabled(bool_vec);
    args.clear();

    match sel_task {
        gui::Tasks::EnergyCalibration => {set_enabled(bool_vec, vec![3,5]); args.push(String::from("ene"))},
        gui::Tasks::PlotSpectrum => {set_enabled(bool_vec, vec![3, 6]); args.push(String::from("plo"))},
        gui::Tasks::CalculatePrimarySpectrum => {set_enabled(bool_vec, vec![0, 6]); args.push(String::from("pri"))},
        gui::Tasks::CalculateFullSpectrum => {set_enabled(bool_vec, vec![0, 2, 6]); args.push(String::from("calc"))},
        gui::Tasks::Calibrate => {set_enabled(bool_vec, vec![0, 1, 2, 5]); args.push(String::from("cal"))},
        gui::Tasks::Quantify => {set_enabled(bool_vec, vec![0, 1, 3, 5, 6]); args.push(String::from("qua"))},
        gui::Tasks::Evaluate => {set_enabled(bool_vec, vec![0, 1, 2, 4, 5]); args.push(String::from("eva"))},
        gui::Tasks::Map => {set_enabled(bool_vec, vec![0, 1, 3, 4, 5]); args.push(String::from("map"))},
        gui::Tasks::CompareMeasuredCalculated => {set_enabled(bool_vec, vec![0, 2, 3, 6]); args.push(String::from("com"))},
        gui::Tasks::FitOneStandardWithPlot => {set_enabled(bool_vec, vec![0, 2, 5, 6]); args.push(String::from("fits"))}, // NOTE this doesn't show in PIQUANT help
        gui::Tasks::BulkSumAndMaxValue => {set_enabled(bool_vec, vec![0, 1, 3, 5, 6]); args.push(String::from("sum"))},
        gui::Tasks::OpticResponse => {set_enabled(bool_vec, vec![0, 2, 3, 5, 6]); args.push(String::from("opt"))},
        gui::Tasks::None => {set_all_disabled(bool_vec)}
    }
}

        // [0,           1,          2,         3,             4,        5,                6,         7]
        // [config_file, calib_file, stds_file, spectrum_file, map_file, element_controls, plot_file, execute_button]

/// Set textbox color depending if path is valid or not
pub fn set_valid_path_colors(text_vec: Vec<String>, color_vec: &mut Vec<Color32>) {
    let mut index = 0;
    for text in text_vec.iter() {
        // Check if valid path
        if check_path(text.to_string()) {
            // Set background color to green to indicate valid
            color_vec[index] = Color32::from_rgb(22, 44, 30);
        } else {
            color_vec[index] = eframe::egui::Style::default().visuals.extreme_bg_color;
        }
        index += 1;
    }
}

/// Update arguments based on selected task & supplied parameters
pub fn update_args(sel_task: &mut gui::Tasks, args: &mut Vec<String>, enable_vec: &mut Vec<bool>, text_vec: Vec<String>) {
    // Args contains only the initial command now
    
}